use std::time::Duration;

pub type GameUUID = String;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum PlayerType {
    Human = 0,
    Computer = 1,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Copy)]
pub enum ColorPreference {
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
pub struct GameConfiguration {
    pub creator_name: String,
    pub opponent: PlayerType,
    pub side_selection: ColorPreference,
    pub time_control: Option<TimeControl>, // None if unlimited
}
