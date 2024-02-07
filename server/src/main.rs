use futures::{FutureExt, StreamExt};
use http::{Method, StatusCode};
use log::{error, info, warn};
use serde_json::from_str;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    filters::ws::{Message, WebSocket},
    reply::json,
    Filter,
};
use warp::{reject::Rejection, reply::Reply};

// ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ---- //
//                                        HTTP                                        //
// ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ---- //

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterRequest {
    user_uuid: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RegisterResponse {
    url: String,
}

// ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ---- //

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Request {
    CreateGame {},
    JoinGame {},
    GetAvailableGames {},
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    GameCreated { game_uuid: String },
    AvailableGames { available_games: Vec<String> },
}

// ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ---- //

#[derive(Debug, Clone)]
pub struct Client {
    pub user_uuid: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Game {
    pub player_white_uuid: Option<String>,
    pub player_black_uuid: Option<String>,
}
type Games = Arc<Mutex<HashMap<String, Game>>>;

type Result<T> = std::result::Result<T, Rejection>;

// ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ----  ---- ---- ---- ---- //

pub async fn health_handler() -> Result<impl Reply> {
    info!("[health_handler]");
    Ok(StatusCode::OK)
}

async fn process_client_msg(client_uuid: &str, msg: Message, clients: &Clients, games: &Games) {
    info!("[process_client_msg]: {:?}", msg);

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
        Request::CreateGame {} => {
            let game_uuid: String = Uuid::new_v4().simple().to_string();
            games.lock().await.insert(
                game_uuid.clone(),
                Game {
                    player_white_uuid: None,
                    player_black_uuid: None,
                },
            );
            Response::GameCreated { game_uuid }
        }
        Request::GetAvailableGames {} => {
            let test: Vec<String> = games
                .lock()
                .await
                .iter_mut()
                .map(|(_, game)| game.player_white_uuid.clone().unwrap_or("1".to_string()))
                .collect();
            Response::AvailableGames {
                available_games: test,
            }
        }
        Request::JoinGame {} => {
            todo!("implement")
        }
    };

    // debug!("[process_client_msg]: current games {:?}", games);

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

    let user_uuid = body.user_uuid;
    let client_uuid = Uuid::new_v4().simple().to_string();

    clients.lock().await.insert(
        client_uuid.clone(),
        Client {
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
