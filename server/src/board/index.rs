use super::board_side::BoardSide;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Index {
    pub b: BoardSide,
    pub x: u8,
    pub y: u8,
    pub z: u8,
}
