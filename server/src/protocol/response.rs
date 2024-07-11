use crate::{
    board::board_state::BoardState,
    state::{
        client::{ClientRole, ClientUUID},
        game_description::{GameDescription, GameUUID},
    },
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    ChangeName {
        status: u8, // TODO?: status u8 -> enum
        user_name: String,
        client_uuid: ClientUUID,
    },

    JoinGame {
        status: u8, // TODO?: status u8 -> enum
        client_uuid: ClientUUID,
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
        game_state: BoardState,
    }, // TODO: rename [BoardState]
}
