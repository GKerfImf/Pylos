use futures::{FutureExt, StreamExt};
use http::{Method, StatusCode};
use log::{debug, error, info, warn};
use rand::Rng;
use serde_json::from_str;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    filters::ws::{Message, WebSocket},
    reply::{json, Reply},
    Filter,
};
use pylos::{
    board::board_state::initialize_board_state,
    game::{
        client::{Client, ClientUUID, Clients, UserUUID},
        game::{Game, Games},
    },
    protocol::{
        html::{RegisterRequest, RegisterResponse},
        request::Request,
        response::Response,
        result::Result,
    },
};


pub async fn health_handler() -> Result<impl Reply> {
    info!("[health_handler]");
    Ok(StatusCode::OK)
}

fn initialize_game_state() -> Game {
    Game {
        watching: vec![],
        player_white: None,
        player_black: None,
        state: initialize_board_state(),
    }
}

async fn process_client_msg(client_uuid: &str, msg: Message, clients: &Clients, games: &Games) {
    info!("[process_client_msg]: {:?} {:?}", client_uuid, msg);

    // Parse the message string into a `Request` enum.
    let req: Request = match msg.to_str() {
        Ok(message) => match from_str(&message) {
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
    info!("[process_client_msg]: received request {:?}", req);

    let res: Response = match req {
        // TODO: make it a separate function
        Request::ChangeName { new_user_name } => {
            match clients.lock().await.get_mut(client_uuid) {
                Some(client) => {
                    client.user_name = new_user_name.clone();
                }
                None => {
                    warn!("Client UUID does not exist, ignore"); // TODO
                    return;
                }
            }
            Response::ChangeName {
                status: 200,
                client_uuid: client_uuid.to_string(),
                user_name: new_user_name,
            }
        }

        // TODO: make it a separate function
        Request::GetClientName { client_uuid } => {
            match clients.lock().await.get(&client_uuid) {
                Some(client) => Response::ClientName {
                    client_uuid: client_uuid.to_string(),
                    user_name: client.clone().user_name,
                },
                None => {
                    warn!("Client UUID does not exist, ignore"); // TODO
                    return;
                }
            }
        }

        // TODO: make it a separate function
        Request::CreateGame {} => {
            let user_name: String = clients
                .lock()
                .await
                .get(client_uuid)
                .expect("Client UUID does not exist")
                .user_name
                .clone();
            let game_uuid: String = Uuid::new_v4().simple().to_string();
            games
                .lock()
                .await
                .insert(game_uuid.clone(), initialize_game_state());
            Response::CreateGame {
                status: 200,
                user_name,
                game_uuid,
            }
        }
        // TODO: make it a separate function
        Request::GetAvailableGames {} => {
            let uuids: Vec<String> = games
                .lock()
                .await
                .iter_mut()
                .map(|(game_uuid, _)| game_uuid.clone())
                .collect();
            Response::AvailableGames { game_uuids: uuids }
        }
        // TODO: make it a separate function
        Request::SetGameState {
            game_uuid,
            game_state,
        } => {
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
            Response::GameState {
                game_state: game_state,
            }
        }
        // TODO: make it a separate function
        Request::GetGameState { game_uuid } => {
            // TODO: fix unwraps and clones
            Response::GameState {
                game_state: games.lock().await.get(&game_uuid).unwrap().state.clone(),
            }
        }
        // TODO: make it a separate function
        Request::JoinGame { game_uuid } => {
            let mut locked = games.lock().await;
            match locked.get_mut(&game_uuid) {
                Some(game) => {
                    game.watching.push(client_uuid.to_string());

                    if game.watching.len() == 2 {
                        // TODO: For now, let's assume that we always assign random colors to players
                        assert!(game.player_white == None);
                        assert!(game.player_black == None);

                        let mut rng = rand::thread_rng();
                        let r: usize = rng.gen();
                        let (iw, ib) = ((0 + r) % 2, (1 + r) % 2);

                        game.player_white = Some(game.watching[iw].clone());
                        game.player_black = Some(game.watching[ib].clone());
                    }
                }
                None => {
                    warn!("Game uuid does not exists {}", game_uuid);
                    return;
                }
            };
            debug!("{:?}", locked.clone());

            // TODO: send game state only to [watch] list
            Response::JoinGame {
                status: 200,
                client_uuid: client_uuid.to_string(),
                game_uuid,
            }
        }
    };

    // TODO: del
    clients.lock().await.iter_mut().for_each(|(_, client)| {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
        }
    });
}

pub async fn client_connection(
    ws: WebSocket,
    client_uuid: String,
    clients: Clients,
    games: Games,
    mut client: Client,
) -> () {
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

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    info!("[register_handler]: {:?} ", body);

    let user_name: String = body.user_name;
    let user_uuid: UserUUID = body.user_uuid;
    let client_uuid: ClientUUID = Uuid::new_v4().simple().to_string();

    clients.lock().await.insert(
        client_uuid.clone(),
        Client {
            user_name,
            user_uuid: user_uuid.clone(),
            sender: None,
        },
    );

    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", client_uuid),
    }))
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    info!("[unregister_handler]: {:?}", id);
    clients.lock().await.remove(&id);
    Ok(StatusCode::OK)
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_games(games: Games) -> impl Filter<Extract = (Games,), Error = Infallible> + Clone {
    warp::any().map(move || games.clone())
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let games: Games = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path("health")
        .and(warp::post())
        .and_then(health_handler);

    let users = warp::path("clients");
    let users_routes = users
        .and(warp::post())
        .and(warp::body::json())
        // .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and_then(register_handler)
        .or(users
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(unregister_handler));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and(with_games(games.clone()))
        .and_then(ws_handler);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_headers(vec![
            "Origin",
            "User-Agent",
            "Content-Type",
            "Access-Control-Allow-Origin",
        ])
        .allow_methods(&[
            Method::POST,
            Method::PUT,
            Method::GET,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .max_age(30)
        .build();

    let routes = health_route.or(users_routes).or(ws_route).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
