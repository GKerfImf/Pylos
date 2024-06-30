use crate::board::{board_side::BoardSide, index::Index};
use super::{ball::Ball, player_side::PlayerSide};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BoardState {
    pub nmove: u8,
    pub turn: PlayerSide,
    pub takeDownRule: u8,                // TODO: rename
    pub selectedBall: Option<Ball>,      // TODO: rename
    pub selectedGhostBall: Option<Ball>, // TODO: rename
    pub balls: Vec<Ball>,
}

pub fn initialize_board_state() -> BoardState {
    let balls: Vec<Ball> = (0..5)
        .flat_map(|x| {
            (0..3)
                .flat_map(|y| {
                    vec![
                        Ball {
                            player: PlayerSide::White,
                            index: Index {
                                b: BoardSide::White,
                                x: x,
                                y: y,
                                z: 0,
                            },
                        },
                        Ball {
                            player: PlayerSide::Black,
                            index: Index {
                                b: BoardSide::Black,
                                x: x,
                                y: y,
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
