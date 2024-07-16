use super::{board_side::BoardSide, index::Index, player_side::PlayerSide};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub player: PlayerSide,
    pub index: Index,
}

impl Ball {
    pub fn new(player: PlayerSide, index: Index) -> Self {
        Ball { player, index }
    }

    // wr = White ball in Reserve
    pub fn new_wr(x: i8, y: i8) -> Self {
        Ball::new(
            PlayerSide::White,
            Index {
                b: BoardSide::White,
                x,
                y,
                z: 0,
            },
        )
    }

    // wc = White ball in Center
    pub fn new_wc(x: i8, y: i8, z: i8) -> Self {
        Ball::new(
            PlayerSide::White,
            Index {
                b: BoardSide::Center,
                x,
                y,
                z,
            },
        )
    }

    // br = Black ball in Reserve
    pub fn new_br(x: i8, y: i8) -> Self {
        Ball::new(
            PlayerSide::Black,
            Index {
                b: BoardSide::Black,
                x,
                y,
                z: 0,
            },
        )
    }

    // bc = Black ball in Center
    pub fn new_bc(x: i8, y: i8, z: i8) -> Self {
        Ball::new(
            PlayerSide::Black,
            Index {
                b: BoardSide::Center,
                x,
                y,
                z,
            },
        )
    }
}
