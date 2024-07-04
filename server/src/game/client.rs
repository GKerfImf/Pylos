use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::filters::ws::Message;

pub type UserUUID = String;
pub type ClientUUID = String;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum ClientRole {
    PlayerBlack = 0,
    PlayerWhite = 1,
    Viewer = 2,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub user_name: String,
    pub user_uuid: UserUUID,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
pub type Clients = Arc<Mutex<HashMap<ClientUUID, Client>>>;
