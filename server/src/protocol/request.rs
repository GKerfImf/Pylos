use crate::{
    board::board_state::BoardState,
    game::{client::ClientUUID, game::GameUUID},
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum Request {
    ChangeName {
        new_user_name: String,
    },
    GetClientName {
        client_uuid: ClientUUID,
    },

    CreateGame {},
    JoinGame {
        game_uuid: GameUUID,
    },
    GetAvailableGames {},
    GetGameState {
        game_uuid: GameUUID,
    },
    SetGameState {
        // TODO: rename [SetBoardState]
        game_uuid: GameUUID,
        game_state: BoardState,
    },
}
