use super::{ball::Ball, player_side::PlayerSide};
use crate::board::{board_side::BoardSide, index::Index};

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BoardState {
    pub nmove: u8,
    pub turn: PlayerSide,
    pub takeDownRule: u8,                // TODO: rename
    pub selectedBall: Option<Ball>,      // TODO: rename
    pub selectedGhostBall: Option<Ball>, // TODO: rename
    pub balls: Vec<Ball>,
}

impl BoardState {
    pub fn new() -> BoardState {
        let balls: Vec<Ball> = (0..5)
            .flat_map(|x| {
                (0..3)
                    .flat_map(|y| {
                        vec![
                            Ball {
                                player: PlayerSide::White,
                                index: Index {
                                    b: BoardSide::White,
                                    x,
                                    y,
                                    z: 0,
                                },
                            },
                            Ball {
                                player: PlayerSide::Black,
                                index: Index {
                                    b: BoardSide::Black,
                                    x,
                                    y,
                                    z: 0,
                                },
                            },
                        ]
                    })
                    .collect::<Vec<Ball>>()
            })
            .collect();

        BoardState {
            nmove: 1,
            turn: PlayerSide::White,
            takeDownRule: 0,
            selectedBall: None,
            selectedGhostBall: None,
            balls,
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}
