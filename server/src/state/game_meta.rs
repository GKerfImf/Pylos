use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum GameState {
    Pending = 0,
    InProgress = 1,
    Completed = 2,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GameMeta {
    pub status: GameState,
    pub created_at: DateTime<Utc>,
    pub last_move_at: Option<DateTime<Utc>>,
}

impl GameMeta {
    pub fn new_pending() -> Self {
        GameMeta {
            status: GameState::Pending,
            created_at: Utc::now(),
            last_move_at: None,
        }
    }

    pub fn update_last_move_at(&mut self) {
        self.last_move_at = Some(Utc::now());
    }

    pub fn promote_to_in_progress(&mut self) {
        self.status = GameState::InProgress;
    }

    pub fn promote_to_completed(&mut self) {
        self.status = GameState::Completed;
    }
}
