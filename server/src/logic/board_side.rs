use std::fmt;

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BoardSide {
    White = 0,
    Black = 1,
    Center = 2,
}

impl fmt::Display for BoardSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match *self {
            BoardSide::White => {
                write!(f, "↘")
            }
            BoardSide::Center => {
                write!(f, "")
            }
            BoardSide::Black => {
                write!(f, "↙")
            }
        }
    }
}
