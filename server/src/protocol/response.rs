use crate::{
    logic::board::BoardFrontend,
    state::{
        client::{ClientRole, UserUUID},
        game_description::{GameDescription, GameUUID},
    },
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    ChangeName {
        status: u8, // TODO?: status u8 -> enum
        user_name: String,
        client_uuid: UserUUID,
    },

    JoinGame {
        status: u8, // TODO?: status u8 -> enum
        client_uuid: UserUUID,
        client_role: ClientRole,
        game_uuid: GameUUID,
    },

    GameParticipants {
        participants: Vec<(String, ClientRole)>, // Vec<Client Name × Client Role>
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
