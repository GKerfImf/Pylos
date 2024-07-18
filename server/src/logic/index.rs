use std::fmt;

use super::board_side::BoardSide;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Index {
    pub b: BoardSide,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}{}", self.b, self.x, self.y, self.z)
    }
}

impl Index {
    pub fn new_c(x: i8, y: i8, z: i8) -> Self {
        let b = BoardSide::Center;
        Index { b, x, y, z }
    }
}
