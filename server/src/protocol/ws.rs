use crate::{
    logic::{amove::Move, board::BoardFrontend},
    protocol::{request::Request, response::Response, result::Result},
    state::{
        client::{Client, Clients},
        game::{Game, Games},
        game_description::{GameDescription, GameUUID, PlayerType},
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

async fn change_profile_info(
    new_user_name: String,
    new_user_avatar: String,
    client_uuid: &str,
    clients: &Clients,
) {
    match clients.lock().await.get_mut(client_uuid) {
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
        client_uuid: client_uuid.to_string(),
        user_name: new_user_name,
        user_avatar: new_user_avatar,
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn create_game(
    game_description: GameDescription,
    client_uuid: &str,
    clients: &Clients,
    games: &Games,
) -> GameUUID {
    let game_uuid: String = Uuid::new_v4().simple().to_string();
    let game = Game::new(game_uuid.clone(), client_uuid.to_string(), game_description);

    games.lock().await.insert(game_uuid.clone(), game);
    let res = Response::CreateGame {
        status: 200,
        game_uuid: game_uuid.clone(),
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
    game_uuid
}

async fn get_available_games(client_uuid: &str, clients: &Clients, games: &Games) {
    let available_games: Vec<(GameUUID, GameDescription)> = games
        .lock()
        .await
        .iter_mut()
        .map(|(game_uuid, game)| (game_uuid.clone(), game.get_description().clone()))
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

async fn join_game(client_uuid: &str, game_uuid: String, _clients: &Clients, games: &Games) {
    let mut locked = games.lock().await;
    match locked.get_mut(&game_uuid) {
        Some(game) => {
            let _ = game.add_client(client_uuid);
        }
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
        }
    };
}

async fn emit_participants(game_uuid: GameUUID, clients: &Clients, games: &Games) {
    let (uuid_player_white, uuid_player_black) = {
        let mut locked = games.lock().await;
        match locked.get_mut(&game_uuid) {
            Some(game) => (game.get_players().0.clone(), game.get_players().1.clone()),
            None => {
                warn!("Game uuid does not exist: {}", game_uuid);
                return;
            }
        }
    };

    let (player_white, player_black) = {
        let locked = clients.lock().await;

        let white = if let Some((uuid, player)) = uuid_player_white {
            Some((
                locked
                    .get(&uuid)
                    .map(|client| client.user_name.clone())
                    .unwrap_or("Disconnected...".to_owned()),
                locked
                    .get(&uuid)
                    .map(|client| client.user_avatar_uuid.clone())
                    .unwrap_or("000".to_owned()),
                player,
            ))
        } else {
            None
        };

        let black = if let Some((uuid, player)) = uuid_player_black {
            Some((
                locked
                    .get(&uuid)
                    .map(|client| client.user_name.clone())
                    .unwrap_or("Disconnected...".to_owned()),
                locked
                    .get(&uuid)
                    .map(|client| client.user_avatar_uuid.clone())
                    .unwrap_or("000".to_owned()),
                player,
            ))
        } else {
            None
        };

        (white, black)
    };

    let res = Response::GameParticipants {
        game_uuid,
        player_white,
        player_black,
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn get_game_state(game_uuid: String, clients: &Clients, games: &Games) {
    let mut locked = games.lock().await;
    let board = match locked.get_mut(&game_uuid) {
        Some(game) => game.get_board().clone(),
        None => {
            warn!("Game uuid does not exist: {}", game_uuid);
            return;
        }
    };

    let res = Response::GameState {
        game_uuid,
        game_state: BoardFrontend::new(board),
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn make_move(
    mv: Move,
    game_uuid: String,
    _client_uuid: &str,
    clients: &Clients,
    games: &Games,
) {
    // TODO: validate [client_uuid] has rights to make a move

    let mut locked = games.lock().await;
    let res: Response = match locked.get_mut(&game_uuid) {
        Some(game) => {
            let _ = game.mut_get_board().make_move(mv); // TODO: handle the result
            Response::GameState {
                game_uuid,
                game_state: BoardFrontend::new(game.get_board().clone()),
            }
        }
        None => {
            warn!("Game uuid does not exists {}", game_uuid);
            return;
        }
    };

    // TODO: do not send response to everyone
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

async fn current_opponent_type_turn(game_uuid: String, games: &Games) -> Option<PlayerType> {
    games
        .lock()
        .await
        .get(&game_uuid)
        .expect("Game UUID does not exist")
        .opponent_type_turn()
}

async fn make_ai_move(game_uuid: String, clients: &Clients, games: &Games) {
    let res: Response = match games.lock().await.get_mut(&game_uuid) {
        Some(game) => Response::GameState {
            game_uuid,
            game_state: BoardFrontend::new(game.make_ai_move().clone()),
        },
        None => {
            warn!("Game uuid does not exists {}", game_uuid);
            return;
        }
    };

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
        Request::CreateGame { game_description } => {
            let game_uuid = create_game(game_description, client_uuid, clients, games).await;
            join_game(client_uuid, game_uuid.clone(), clients, games).await;
            emit_participants(game_uuid.clone(), clients, games).await;
            if let Some(PlayerType::Computer) =
                current_opponent_type_turn(game_uuid.clone(), games).await
            {
                make_ai_move(game_uuid, clients, games).await;
            }
        }
        Request::GetAvailableGames {} => get_available_games(client_uuid, clients, games).await,
        Request::JoinGame { game_uuid } => {
            join_game(client_uuid, game_uuid.clone(), clients, games).await;
            emit_participants(game_uuid, clients, games).await;

            // TODO: also update participants when someone leaves
        }
        Request::GetGameState { game_uuid } => get_game_state(game_uuid, clients, games).await,
        Request::MakeMove { game_uuid, mv } => {
            make_move(mv, game_uuid.clone(), client_uuid, clients, games).await;
            if let Some(PlayerType::Computer) =
                current_opponent_type_turn(game_uuid.clone(), games).await
            {
                make_ai_move(game_uuid, clients, games).await;
            }
        }
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
