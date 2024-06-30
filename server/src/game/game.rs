use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::client::ClientUUID;
use crate::board::board_state::{initialize_board_state, BoardState};

pub type GameUUID = String;

#[derive(Debug, Clone)]
pub struct Game {
    pub watching: Vec<ClientUUID>, // TODO: vec -> set
    pub player_white: Option<ClientUUID>,
    pub player_black: Option<ClientUUID>,
    pub state: BoardState,
}
pub type Games = Arc<Mutex<HashMap<GameUUID, Game>>>;

pub fn initialize_game_state() -> Game {
    Game {
        watching: vec![],
        player_white: None,
        player_black: None,
        state: initialize_board_state(),
    }
}
