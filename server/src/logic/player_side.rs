use std::ops::Not;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlayerSide {
    White = 0,
    Black = 1,
}

impl Not for PlayerSide {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PlayerSide::White => PlayerSide::Black,
            PlayerSide::Black => PlayerSide::White,
        }
    }
}
