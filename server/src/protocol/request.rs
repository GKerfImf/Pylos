use crate::{
    logic::amove::Move,
    state::{game_configuration::GameConfiguration, game_uuid::GameUUID},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Request {
    ChangeProfileInfo {
        new_user_name: String,
        new_user_avatar: String,
    },

    CreateGame {
        game_configuration: GameConfiguration,
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
