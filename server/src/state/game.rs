use rand::Rng;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use super::{
    client::UserUUID,
    game_description::{ColorPreference, GameDescription, GameUUID, PlayerType},
};
use crate::logic::{ai::AI, board::Board, player_side::PlayerSide};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Role {
    Player = 0,
    Spectator = 1,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Player {
    time_left: (), // TODO
    player_type: PlayerType,
}

#[derive(Debug, Clone)]
pub struct Game {
    _creator_uuid: UserUUID,
    _game_uuid: GameUUID,

    player_white: Option<(UserUUID, Player)>, 
    player_black: Option<(UserUUID, Player)>, 
    spectators: Vec<UserUUID>,
    board: Board,

    game_description: GameDescription,
}
pub type Games = Arc<Mutex<HashMap<GameUUID, Game>>>;

impl Game {
    pub fn new(
        game_uuid: GameUUID,
        client_uuid: UserUUID,
        game_description: GameDescription,
    ) -> Game {
        Game {
            _creator_uuid: client_uuid,
            _game_uuid: game_uuid,
            player_white: None,
            player_black: None,
            spectators: vec![],
            board: Board::new(),

            game_description,
        }
    }

    pub fn get_description(&self) -> &GameDescription {
        &self.game_description
    }

    #[allow(clippy::type_complexity)]
    pub fn get_players(&self) -> (&Option<(UserUUID, Player)>, &Option<(UserUUID, Player)>) {
        (&self.player_white, &self.player_black)
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn mut_get_board(&mut self) -> &mut Board {
        &mut self.board
    }

    fn get_ai_color(&self) -> Option<PlayerSide> {
        if let Some((_, player)) = &self.player_white {
            if player.player_type == PlayerType::Computer {
                return Some(PlayerSide::White);
            }
        };
        if let Some((_, player)) = &self.player_black {
            if player.player_type == PlayerType::Computer {
                return Some(PlayerSide::Black);
            }
        };
        None
    }

    pub fn make_ai_move(&mut self) -> Board {
        let mut ai = AI {
            side: self.get_ai_color().expect("No AI found"),
            board: self.get_board().clone(),
        };

        // TODO: make a move, not rewrite the board
        self.board = ai.minmax_moves();
        self.board.clone()
    }

    fn add_player_helper(&mut self, client_uuid: &str, player: Player, color: ColorPreference) {
        match color {
            ColorPreference::AlwaysWhite => {
                self.player_white = Some((client_uuid.to_string(), player));
            }
            ColorPreference::AlwaysBlack => {
                self.player_black = Some((client_uuid.to_string(), player));
            }
            ColorPreference::Random => {
                if rand::thread_rng().gen() {
                    self.player_white = Some((client_uuid.to_string(), player));
                } else {
                    self.player_black = Some((client_uuid.to_string(), player));
                }
            }
        }
    }

    fn add_player(&mut self, client_uuid: &str, player: Player) -> Result<(), &'static str> {
        match (&self.player_white, &self.player_black) {
            (None, None) => {
                self.add_player_helper(client_uuid, player, self.game_description.side_selection);
                if let PlayerType::Computer = self.game_description.opponent {
                    self.add_player(
                        client_uuid,
                        Player {
                            time_left: (),
                            player_type: PlayerType::Computer,
                        },
                    )
                } else {
                    Ok(())
                }
            }
            (None, Some(_)) => {
                self.add_player_helper(client_uuid, player, ColorPreference::AlwaysWhite);
                Ok(())
            }
            (Some(_), None) => {
                self.add_player_helper(client_uuid, player, ColorPreference::AlwaysBlack);
                Ok(())
            }
            (Some(_), Some(_)) => Err(""),
        }
    }

    fn add_spectator(&mut self, client_uuid: &str) -> Result<(), &'static str> {
        self.spectators.push(client_uuid.to_string());
        Ok(())
    }

    pub fn add_client(&mut self, client_uuid: &str) -> Result<(), &'static str> {
        let _ = self.add_spectator(client_uuid);

        if self.player_white.is_none() || self.player_black.is_none() {
            let _ = self.add_player(
                client_uuid,
                Player {
                    time_left: (),
                    player_type: PlayerType::Human,
                },
            );
        };

        Ok(())
    }

    pub fn opponent_type_turn(&self) -> Option<PlayerType> {
        match self.board.get_turn() {
            PlayerSide::White => match self.player_white.clone() {
                Some(player) => Some(player.1.player_type),
                None => None,
            },
            PlayerSide::Black => match self.player_black.clone() {
                Some(player) => Some(player.1.player_type),
                None => None,
            },
        }
    }
}
