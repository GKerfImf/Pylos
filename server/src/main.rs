use http::{Method, StatusCode};
use log::info;
use pylos::{
    protocol::{
        html::{RegisterRequest, RegisterResponse},
        result::Result,
        ws::ws_handler,
    },
    state::{
        client::{Client, Clients},
        game::Games,
        user_uuid::UserUUID,
    },
};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{
    reply::{json, Reply},
    Filter,
};

pub async fn health_handler() -> Result<impl Reply> {
    info!("[health_handler]");
    Ok(StatusCode::OK)
}

pub async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    info!("[register_handler]: {:?} ", body);

    clients.lock().await.insert(
        body.user_uuid.clone(),
        Client {
            user_name: body.user_name,
            user_uuid: body.user_uuid.clone(),
            user_avatar_uuid: body.user_avatar_uuid,
            sender: None,
        },
    );

    Ok(json(&RegisterResponse {
        url: format!("ws://127.0.0.1:8000/ws/{}", body.user_uuid),
    }))
}

pub async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    info!("[unregister_handler]: {:?}", id);
    clients.lock().await.remove(&UserUUID::new(id).unwrap());
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
    std::env::set_var("RUST_LOG", "info");
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
