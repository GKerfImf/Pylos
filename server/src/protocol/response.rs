use crate::{
    logic::board::BoardFrontend,
    state::{
        client::{ClientRole, UserUUID},
        game_description::{GameDescription, GameUUID},
    },
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    ChangeProfileInfo {
        status: u8, // TODO?: status u8 -> enum
        client_uuid: UserUUID,
        user_name: String,
        user_avatar: String,
    },

    JoinGame {
        status: u8, // TODO?: status u8 -> enum
        client_uuid: UserUUID,
        client_role: ClientRole,
        game_uuid: GameUUID,
    },

    GameParticipants {
        // Vec<Client Name × Client Avatar UUID × Client Role>
        participants: Vec<(String, String, ClientRole)>,
        game_uuid: GameUUID,
    },

    CreateGame {
        status: u8,
        user_name: String,
        game_uuid: GameUUID,
    },

    AvailableGames {
        game_descriptions: Vec<GameDescription>,
    },
    GameState {
        game_state: BoardFrontend,
    },
}
