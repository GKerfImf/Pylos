use std::time::Duration;

pub type GameUUID = String;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum SideSelection {
    AlwaysWhite = 0,
    AlwaysBlack = 1,
    Random = 2,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TimeControl {
    pub time: Duration,
    pub increment: Duration,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GameDescription {
    pub game_uuid: GameUUID,
    pub creator_name: String,
    pub side_selection: SideSelection,
    pub time_control: Option<TimeControl>, // None if unlimited
}
