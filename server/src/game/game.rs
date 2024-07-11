use rand::Rng;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::{
    client::{ClientRole, ClientUUID},
    game_description::{GameDescription, GameUUID},
};
use crate::board::board_state::{initialize_board_state, BoardState};

#[derive(Debug, Clone)]
pub struct Game {
    pub players: Vec<ClientUUID>,
    pub player_white: u8,
    pub player_black: u8,          // TODO : del
    pub watching: Vec<ClientUUID>, // TODO: vec -> set

    pub state: BoardState,

    pub game_description: GameDescription,
}
pub type Games = Arc<Mutex<HashMap<GameUUID, Game>>>;

impl Game {
    pub fn new(client_uuid: ClientUUID, game_description: GameDescription) -> Game {
        // TODO: For now, let's assume that we always assign random colors to players
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen();

        Game {
            players: vec![],
            player_white: r % 2,
            player_black: (r + 1) % 2,

            watching: vec![],
            state: initialize_board_state(),
            game_description,
        }
    }

    pub fn get_participants(&self) -> Vec<(ClientUUID, ClientRole)> {
        let player_colors = if self.player_white == 0 {
            vec![ClientRole::PlayerBlack, ClientRole::PlayerWhite]
        } else {
            vec![ClientRole::PlayerWhite, ClientRole::PlayerBlack]
        };

        let pl: Vec<(ClientUUID, ClientRole)> = self
            .players
            .iter()
            .zip(player_colors.iter())
            .map(|(player, role)| (player.clone(), role.clone()))
            .collect();

        let wt: Vec<(ClientUUID, ClientRole)> = self
            .watching
            .iter()
            .map(|name| (name.clone(), ClientRole::Viewer))
            .collect();

        return [pl, wt].concat();
    }
}
