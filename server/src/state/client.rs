use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::filters::ws::Message;

pub type UserUUID = String;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_name: String,
    pub user_uuid: UserUUID,
    pub user_avatar_uuid: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}
pub type Clients = Arc<Mutex<HashMap<UserUUID, Client>>>;
