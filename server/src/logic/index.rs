use super::board_side::BoardSide;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Index {
    pub b: BoardSide,
    pub x: i8,
    pub y: i8,
    pub z: i8,
}
