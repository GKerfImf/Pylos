use crate::{
    logic::board::BoardFrontend,
    state::{
        game::Player, game_configuration::GameConfiguration, game_meta::GameMeta,
        game_uuid::GameUUID,
    },
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    ChangeProfileInfo {
        status: u8,
        user_name: String,
        user_avatar: String,
    },

    CreateGame {
        status: u8,
        game_uuid: GameUUID,
    },

    GameParticipants {
        game_uuid: GameUUID,
        player_white: Option<(String, String, Player)>, // Name, AvatarUUID, Player
        player_black: Option<(String, String, Player)>, // Name, AvatarUUID, Player
    },

    AvailableGames {
        available_games: Vec<(GameUUID, GameMeta, GameConfiguration)>,
    },

    GameState {
        game_uuid: GameUUID,
        game_state: BoardFrontend,
    },
}
