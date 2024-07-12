use std::time::Duration;

use crate::{
    board::board_state::BoardState,
    protocol::{request::Request, response::Response, result::Result},
    state::{
        client::{Client, ClientRole, Clients, UserUUID},
        game::{Game, Games},
        game_description::{GameDescription, GameUUID, SideSelection, TimeControl},
    },
};
use futures::{FutureExt, StreamExt};
use log::{error, info, warn};
use serde_json::from_str;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    filters::ws::{Message, WebSocket},
    reply::Reply,
};

async fn change_name(new_user_name: String, client_uuid: &str, clients: &Clients) {
    match clients.lock().await.get_mut(client_uuid) {
        Some(client) => {
            client.user_name = new_user_name.clone();
        }
        None => {
            warn!("Client UUID does not exist, ignore"); // TODO
            return;
        }
    }

    let res = Response::ChangeName {
        status: 200,
        client_uuid: client_uuid.to_string(),
        user_name: new_user_name,
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn create_game(
    time: u64,
    increment: u64,
    client_uuid: &str,
    clients: &Clients,
    games: &Games,
) {
    let user_name: String = clients
        .lock()
        .await
        .get(client_uuid)
        .expect("Client UUID does not exist")
        .user_name
        .clone();

    let game_uuid: String = Uuid::new_v4().simple().to_string();
    let game_description = GameDescription {
        game_uuid: game_uuid.clone(),
        creator_name: user_name.clone(),
        side_selection: SideSelection::Random,
        time_control: TimeControl {
            time: Duration::from_secs(time),
            increment: Duration::from_secs(increment),
        },
    };

    let game = Game::new(client_uuid.to_string(), game_description);

    games.lock().await.insert(game_uuid.clone(), game);

    let res = Response::CreateGame {
        status: 200,
        user_name,
        game_uuid,
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn get_available_games(client_uuid: &str, clients: &Clients, games: &Games) {
    let game_descriptions: Vec<GameDescription> = games
        .lock()
        .await
        .iter_mut()
        .map(|(_, game)| game.game_description.clone())
        .collect();

    let res = Response::AvailableGames { game_descriptions };

    clients
        .lock()
        .await
        .get(client_uuid)
        .expect("Client UUID does not exist")
        .sender
        .iter()
        .for_each(|sender| {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        });
}

async fn join_game(client_uuid: &str, game_uuid: String, clients: &Clients, games: &Games) {
    let mut locked = games.lock().await;
    let client_role = match locked.get_mut(&game_uuid) {
        Some(game) => {
            if game.players.len() < 2 {
                game.players.push(client_uuid.to_string());
                let pick_side_aux =
                    (game.players.len() == 2) ^ (game.player_white < game.player_black);
                if pick_side_aux {
                    ClientRole::PlayerWhite
                } else {
                    ClientRole::PlayerBlack
                }
            } else {
                game.watching.push(client_uuid.to_string());
                ClientRole::Viewer
            }
        }
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
            return;
        }
    };

    drop(locked);

    let res = Response::JoinGame {
        status: 200,
        client_uuid: client_uuid.to_string(),
        client_role,
        game_uuid,
    };

    clients
        .lock()
        .await
        .get(client_uuid)
        .expect("Client UUID does not exist")
        .sender
        .iter()
        .for_each(|sender| {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        });
}

async fn broadcast_participants(game_uuid: GameUUID, clients: &Clients, games: &Games) {
    let locked = games.lock().await;
    let participants: Vec<(UserUUID, ClientRole)> = match locked.get(&game_uuid) {
        Some(game) => game.get_participants(),
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
            return;
        }
    };
    drop(locked);

    let locked = clients.lock().await;

    let res = Response::GameParticipants {
        participants: participants
            .iter()
            .filter_map(|(uuid, role)| {
                locked
                    .get(uuid)
                    .map(|client| (client.user_name.clone(), role.clone()))
            })
            .collect(),
        game_uuid,
    };

    // TODO: do not send response to everyone
    locked.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn get_game_state(game_uuid: String, clients: &Clients, games: &Games) {
    let res = Response::GameState {
        game_state: games.lock().await.get(&game_uuid).unwrap().state.clone(),
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn set_game_state(
    game_uuid: String,
    game_state: BoardState,
    clients: &Clients,
    games: &Games,
) {
    // Validation
    let current_game_state = games.lock().await.get(&game_uuid).unwrap().state.clone();
    if current_game_state.nmove >= game_state.nmove {
        warn!("Received a bad update, reject"); // TODO
        return;
    }

    // Update
    let mut locked = games.lock().await;
    match locked.get_mut(&game_uuid) {
        Some(game) => {
            game.state = game_state.clone();
        }
        None => {
            warn!("Game uuid does not exists {}", game_uuid);
            return;
        }
    };

    // TODO: send game state only to [watch] list
    let res = Response::GameState { game_state };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

// TODO: use proper type for [client_uuid]
async fn process_client_msg(client_uuid: &str, msg: Message, clients: &Clients, games: &Games) {
    // Parse the message string into a `Request` enum.
    let req: Request = match msg.to_str() {
        Ok(message) => match from_str(message) {
            Ok(req) => req,
            Err(e) => {
                warn!("error while parsing message to topics request: {}", e);
                return;
            }
        },
        Err(_) => {
            warn!("error while parsing message to string");
            return;
        }
    };
    // info!("[process_client_msg]: received request {:?}", req);

    match req {
        Request::ChangeName { new_user_name } => {
            change_name(new_user_name, client_uuid, clients).await
        }
        Request::CreateGame {
            opponent: _,
            side: _,
            time,
            increment,
        } => create_game(time * 60, increment, client_uuid, clients, games).await,
        Request::GetAvailableGames {} => get_available_games(client_uuid, clients, games).await,
        Request::JoinGame { game_uuid } => {
            join_game(client_uuid, game_uuid.clone(), clients, games).await;
            broadcast_participants(game_uuid, clients, games).await
            // TODO: also update participants when someone leaves
        }
        Request::GetGameState { game_uuid } => get_game_state(game_uuid, clients, games).await,
        Request::SetGameState {
            game_uuid,
            game_state,
        } => set_game_state(game_uuid, game_state, clients, games).await,
    };
}

async fn client_connection(
    ws: WebSocket,
    client_uuid: String,
    clients: Clients,
    games: Games,
    mut client: Client,
) {
    info!("[client_connection]: {}", client_uuid);
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.lock().await.insert(client_uuid.clone(), client);

    info!("[client_connection]: Client {} connected", client_uuid);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                error!(
                    "error receiving ws message for id: {}): {}",
                    client_uuid.clone(),
                    e
                );
                break;
            }
        };
        process_client_msg(&client_uuid, msg, &clients, &games).await;
    }

    clients.lock().await.remove(&client_uuid);
    info!("[client_connection]: Client {} disconnected", client_uuid);
}

pub async fn ws_handler(
    ws: warp::ws::Ws,
    client_uuid: String,
    clients: Clients,
    games: Games,
) -> Result<impl Reply> {
    info!("[ws_handler]: {}", client_uuid);
    let client = clients.lock().await.get(&client_uuid).cloned();
    match client {
        Some(client) => Ok(ws.on_upgrade(move |socket| {
            client_connection(socket, client_uuid, clients, games, client)
        })),
        None => Err(warp::reject::not_found()),
    }
}
