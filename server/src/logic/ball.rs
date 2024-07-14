use super::{index::Index, player_side::PlayerSide};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy, PartialEq)]
pub struct Ball {
    pub player: PlayerSide,
    pub index: Index,
}
