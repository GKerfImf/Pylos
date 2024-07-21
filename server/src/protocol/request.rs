use crate::{logic::amove::Move, state::game_description::GameUUID};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Request {
    ChangeName {
        new_user_name: String,
    },

    CreateGame {
        opponent: String,     // TODO: proper type
        side: String,         // TODO: proper type
        time_control: String, // TODO: proper type
        time: u64,
        increment: u64,
    },
    JoinGame {
        game_uuid: GameUUID,
    },
    GetAvailableGames {},
    GetGameState {
        game_uuid: GameUUID,
    },

    MakeMove {
        game_uuid: GameUUID,
        mv: Move,
    },
}
