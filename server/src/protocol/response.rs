use crate::{
    board::board_state::BoardState,
    game::{client::ClientUUID, game::GameUUID},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Response {
    ChangeName {
        status: u8, // TODO?: status u8 -> enum
        user_name: String,
        client_uuid: ClientUUID,
    },
    ClientName {
        user_name: String,
        client_uuid: ClientUUID,
    },

    CreateGame {
        status: u8,
        user_name: String,
        game_uuid: GameUUID,
    },
    JoinGame {
        status: u8, // TODO?: status u8 -> enum
        client_uuid: ClientUUID,
        game_uuid: GameUUID,
    },
    AvailableGames {
        game_uuids: Vec<GameUUID>,
    },
    GameState {
        game_state: BoardState,
    }, // TODO: rename [BoardState]
}
