use crate::{
    logic::amove::Move,
    protocol::{request::Request, response::Response, result::Result},
    state::{
        client::{Client, Clients},
        game::{Game, Games},
        game_configuration::GameConfiguration,
        game_meta::GameMeta,
        game_uuid::GameUUID,
        user_uuid::UserUUID,
    },
};
use chrono::{DateTime, Duration, Utc};
use futures::{future::join_all, FutureExt, StreamExt};
use log::{error, info, warn};
use serde_json::from_str;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    filters::ws::{Message, WebSocket},
    reply::Reply,
};

async fn change_profile_info(
    new_user_name: String,
    new_user_avatar: String,
    client_uuid: UserUUID,
    clients: &Clients,
) {
    let mut clients_guard = clients.lock().await;

    match clients_guard.get_mut(&client_uuid) {
        Some(client) => {
            client.user_name.clone_from(&new_user_name);
            client.user_avatar_uuid.clone_from(&new_user_avatar);
        }
        None => {
            warn!("Client UUID does not exist, ignore"); // TODO
            return;
        }
    }

    let res = Response::ChangeProfileInfo {
        status: 200,
        user_name: new_user_name,
        user_avatar: new_user_avatar,
    };

    clients_guard
        .get(&client_uuid)
        .expect("Client UUID does not exist")
        .sender
        .iter()
        .for_each(|sender| {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        });
}

async fn create_game(
    game_configuration: GameConfiguration,
    client_uuid: UserUUID,
    clients: &Clients,
    games: &Games,
) {
    let game_uuid: String = Uuid::new_v4().simple().to_string();
    let game = Game::new(
        game_uuid.clone(),
        client_uuid.clone(),
        game_configuration,
        Arc::clone(clients),
    );

    games.lock().await.insert(game_uuid.clone(), game);
    let res = Response::CreateGame {
        status: 200,
        game_uuid,
    };

    clients
        .lock()
        .await
        .get(&client_uuid)
        .expect("Client UUID does not exist")
        .sender
        .iter()
        .for_each(|sender| {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        });
}

async fn get_available_games(client_uuid: &UserUUID, clients: &Clients, games: &Games) {
    fn is_two_weeks_ago_or_later(date: DateTime<Utc>) -> bool {
        Utc::now() <= date + Duration::weeks(2)
    }

    let available_games: Vec<(GameUUID, GameMeta, GameConfiguration)> = {
        let games_guard = games.lock().await;
        let futures = games_guard.iter().map(|(game_uuid, game)| {
            let game_uuid = game_uuid.clone();
            let game_meta_future = game.get_meta();
            let game_description = game.get_description().clone();

            async move {
                let game_meta = game_meta_future.await;
                (game_uuid, game_meta, game_description)
            }
        });
        join_all(futures).await
    }
    .into_iter()
    .filter(|game| is_two_weeks_ago_or_later(game.1.created_at))
    .collect();

    let res = Response::AvailableGames { available_games };

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

async fn join_game(client_uuid: UserUUID, game_uuid: String, games: &Games) {
    match games.lock().await.get_mut(&game_uuid) {
        Some(game) => {
            game.add_client(client_uuid).await;
        }
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
        }
    };
}

async fn get_game_state(client_uuid: &UserUUID, game_uuid: &String, games: &Games) {
    match games.lock().await.get_mut(game_uuid) {
        Some(game) => game.emit_board(client_uuid).await,
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
        }
    };
}

async fn make_move(mv: Move, game_uuid: &String, client_uuid: &UserUUID, games: &Games) {
    match games.lock().await.get_mut(game_uuid) {
        Some(game) => {
            let _ = game.make_move(client_uuid, mv).await; // TODO: handle the result
        }
        None => {
            warn!("Game uuid does not exists {}", game_uuid);
        }
    };
}

// TODO: use proper type for [client_uuid]
async fn process_client_msg(client_uuid: UserUUID, msg: Message, clients: &Clients, games: &Games) {
    // Parse the message string into a `Request` enum.
    let req: Request = match msg.to_str() {
        Ok(message) => match from_str(message) {
            Ok(req) => req,
            Err(e) => {
                warn!("error while parsing message to topics request: {}", e);
                warn!("{:#?}", msg);
                return;
            }
        },
        Err(_) => {
            warn!("[process_client_msg]: error while parsing message to string: ");
            warn!("{:#?}", msg);
            return;
        }
    };
    info!("[process_client_msg]: {:#?}", req);

    match req {
        Request::ChangeProfileInfo {
            new_user_name,
            new_user_avatar,
        } => change_profile_info(new_user_name, new_user_avatar, client_uuid, clients).await,
        Request::CreateGame { game_configuration } => {
            create_game(game_configuration, client_uuid, clients, games).await;
        }
        Request::GetAvailableGames {} => get_available_games(&client_uuid, clients, games).await,
        Request::JoinGame { game_uuid } => {
            join_game(client_uuid, game_uuid.clone(), games).await;
            // TODO: also update participants when someone leaves
        }
        Request::GetGameState { game_uuid } => {
            get_game_state(&client_uuid, &game_uuid, games).await
        }
        Request::MakeMove { game_uuid, mv } => {
            make_move(mv, &game_uuid, &client_uuid, games).await;
        }
    };
}

async fn client_connection(
    ws: WebSocket,
    client_uuid: UserUUID,
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
        process_client_msg(client_uuid.clone(), msg, &clients, &games).await;
    }

    clients.lock().await.remove(&client_uuid);
    info!("[client_connection]: Client {} disconnected", client_uuid);
}

pub async fn ws_handler(
    ws: warp::ws::Ws,
    client_uuid: UserUUID,
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
