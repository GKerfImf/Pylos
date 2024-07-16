use std::fmt;

use super::{ball::Ball, player_side::PlayerSide};
use crate::logic::{board_side::BoardSide, index::Index};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
pub struct Move {
    pub from: Ball,
    pub to: Ball,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.from.player {
            PlayerSide::White => {
                write!(f, "[◯ : {} => {}]", self.from.index, self.to.index)
            }
            PlayerSide::Black => {
                write!(f, "[● : {} => {}]", self.from.index, self.to.index)
            }
        }
    }
}

#[rustfmt::skip]
impl Move {

    pub fn new(player: PlayerSide, index_from: Index, index_to: Index) -> Self {
        Move {
            from: Ball { player, index: index_from },
            to: Ball { player, index: index_to, },
        }
    }

    // wrc = White from Reserve to Center
    pub fn new_wrc((a, b): (i8, i8), (x, y, z): (i8, i8, i8)) -> Self {
        Move::new(
            PlayerSide::White,
            Index { b: BoardSide::White, x: a, y: b, z: 0 },
            Index { b: BoardSide::Center, x, y, z },
        )
    }

    // wcr = White from Center to Reserve
    pub fn new_wcr((x, y, z): (i8, i8, i8), (a, b): (i8, i8)) -> Self {
        Move::new(
            PlayerSide::White,
            Index { b: BoardSide::Center, x, y, z },
            Index { b: BoardSide::White, x: a, y: b, z: 0 },
        )
    }

    // wcc = White from Center to Center
    pub fn new_wcc((a, b,c ): (i8, i8, i8), (x, y, z): (i8, i8, i8)) -> Self {
        Move::new(
            PlayerSide::White,
            Index { b: BoardSide::Center, x: a, y: b, z: c },
            Index { b: BoardSide::Center, x, y, z },
        )
    }

    // brc = Black from Reserve to Center
    pub fn new_brc((a, b): (i8, i8), (x, y, z): (i8, i8, i8)) -> Self {
        Move::new(
            PlayerSide::Black,
            Index { b: BoardSide::Black, x: a, y: b, z: 0 },
            Index { b: BoardSide::Center, x, y, z },
        )
    }

    // bcr = Black from Center to Reserve
    pub fn new_bcr((x, y, z): (i8, i8, i8), (a, b): (i8, i8)) -> Self {
        Move::new(
            PlayerSide::Black,
            Index { b: BoardSide::Center, x, y, z },
            Index { b: BoardSide::Black, x: a, y: b, z: 0 },
        )
    }

    // bcc = Black from Center to Center
    pub fn new_bcc((a, b,c ): (i8, i8, i8), (x, y, z): (i8, i8, i8)) -> Self {
        Move::new(
            PlayerSide::Black,
            Index { b: BoardSide::Center, x: a, y: b, z: c },
            Index { b: BoardSide::Center, x, y, z },
        )
    }
}
