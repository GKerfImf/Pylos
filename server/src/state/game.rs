use super::{
    client::Clients,
    game_configuration::{ColorPreference, GameConfiguration, PlayerType},
    game_meta::GameMeta,
    game_uuid::GameUUID,
    user_uuid::UserUUID,
};
use crate::{
    logic::{
        ai::AI,
        amove::Move,
        board::{Board, BoardFrontend},
        player_side::PlayerSide,
    },
    protocol::response::Response,
};
use rand::Rng;
use std::{collections::HashMap, sync::Arc};
use tokio::{spawn, sync::Mutex, task};
use warp::filters::ws::Message;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Player {
    time_left: (), // TODO
    player_type: PlayerType,
}

impl Player {
    pub fn new_computer() -> Self {
        Player {
            time_left: (),
            player_type: PlayerType::Computer,
        }
    }

    pub fn new_human() -> Self {
        Player {
            time_left: (),
            player_type: PlayerType::Human,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    _creator_uuid: UserUUID,
    game_uuid: GameUUID,
    clients: Clients,

    player_white: Option<(UserUUID, Player)>,
    player_black: Option<(UserUUID, Player)>,
    spectators: Vec<UserUUID>,
    board: Arc<Mutex<Board>>,

    game_meta: Arc<Mutex<GameMeta>>, // Any change to the board updates the game's metadata, hence [Mutex]
    game_configuration: GameConfiguration,
}
pub type Games = Arc<Mutex<HashMap<GameUUID, Game>>>;

impl Game {
    pub fn new(
        game_uuid: GameUUID,
        client_uuid: UserUUID,
        game_configuration: GameConfiguration,
        clients: Clients,
    ) -> Game {
        Game {
            _creator_uuid: client_uuid,
            game_uuid,
            clients,

            player_white: None,
            player_black: None,
            spectators: vec![],
            board: Arc::new(Mutex::new(Board::new())),

            game_meta: Arc::new(Mutex::new(GameMeta::new_pending())),
            game_configuration,
        }
    }
}

impl Game {
    pub async fn add_client(&mut self, client_uuid: UserUUID) {
        self.add_spectator(client_uuid.clone());

        if self.player_slot_is_available() {
            self.add_player(client_uuid, Player::new_human());

            if self.player_slots_are_taken() {
                self.game_meta.lock().await.promote_to_in_progress();
            };
            self.ping_ai().await;
        }

        self.broadcast_participants().await;
    }

    pub async fn emit_board(&self, client_uuid: &UserUUID) {
        let res = Response::GameState {
            game_uuid: self.game_uuid.clone(),
            game_state: BoardFrontend::new(self.board.lock().await.clone()),
        };

        self.clients
            .lock()
            .await
            .get(&client_uuid)
            .expect("Client UUID does not exist")
            .sender
            .iter()
            .for_each(|sender| {
                let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
            });
    }

    pub async fn make_move(&mut self, _client_uuid: &UserUUID, mv: Move) {
        // TODO: validate [client_uuid] has rights to make a move

        // Execute the move on the board and update the game's meta
        let _ = self.board.lock().await.make_move(mv);
        self.game_meta.lock().await.update_last_move_at();
        if self.board.lock().await.is_game_over() {
            self.game_meta.lock().await.promote_to_completed();
        }
        self.broadcast_board().await;

        // AI broadcasts the board and updates the game's meta, if needed
        self.ping_ai().await;
    }

    pub async fn get_meta(&self) -> GameMeta {
        self.game_meta.lock().await.clone()
    }

    pub fn get_description(&self) -> &GameConfiguration {
        &self.game_configuration
    }
}

impl Game {
    fn player_slot_is_available(&self) -> bool {
        self.player_white.is_none() || self.player_black.is_none()
    }

    fn player_slots_are_taken(&self) -> bool {
        self.player_white.is_some() && self.player_black.is_some()
    }

    fn get_ai_color(&self) -> Option<PlayerSide> {
        if let Some((_, player)) = &self.player_white {
            if player.player_type == PlayerType::Computer {
                return Some(PlayerSide::White);
            }
        };
        if let Some((_, player)) = &self.player_black {
            if player.player_type == PlayerType::Computer {
                return Some(PlayerSide::Black);
            }
        };
        None
    }
}

impl Game {
    async fn broadcast_participants(&self) {
        let clients_guard = self.clients.lock().await;

        let player_white = if let Some((uuid, player)) = &self.player_white {
            if let Some(client) = clients_guard.get(&uuid) {
                Some((
                    client.user_name.clone(),
                    client.user_avatar_uuid.clone(),
                    player.clone(),
                ))
            } else {
                Some((
                    "Disconnected...".to_owned(),
                    "xxxx".to_owned(),
                    player.clone(),
                ))
            }
        } else {
            None
        };

        let player_black = if let Some((uuid, player)) = &self.player_black {
            if let Some(client) = clients_guard.get(&uuid) {
                Some((
                    client.user_name.clone(),
                    client.user_avatar_uuid.clone(),
                    player.clone(),
                ))
            } else {
                Some((
                    "Disconnected...".to_owned(),
                    "xxxx".to_owned(),
                    player.clone(),
                ))
            }
        } else {
            None
        };

        let res = Response::GameParticipants {
            game_uuid: self.game_uuid.clone(),
            player_white,
            player_black,
        };

        clients_guard
            .iter()
            .filter(|(uuid, _)| self.spectators.contains(uuid))
            .for_each(|(_, client)| {
                if let Some(sender) = &client.sender {
                    let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
                }
            });
    }

    async fn broadcast_board(&self) {
        let res: Response = Response::GameState {
            game_uuid: self.game_uuid.clone(),
            game_state: BoardFrontend::new(self.board.lock().await.clone()),
        };

        self.clients
            .lock()
            .await
            .iter_mut()
            .filter(|(uuid, _)| self.spectators.contains(uuid))
            .for_each(|(_, client)| {
                if let Some(sender) = &client.sender {
                    let _ = sender.send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
                }
            });
    }

    async fn opponent_type_turn(&self) -> Option<PlayerType> {
        match self.board.lock().await.get_turn() {
            PlayerSide::White => match self.player_white.clone() {
                Some(player) => Some(player.1.player_type),
                None => None,
            },
            PlayerSide::Black => match self.player_black.clone() {
                Some(player) => Some(player.1.player_type),
                None => None,
            },
        }
    }

    async fn ping_ai(&mut self) {
        if let Some(PlayerType::Computer) = self.opponent_type_turn().await {
            self.trigger_ai_move();
        }
    }

    fn trigger_ai_move(&mut self) {
        let ai_side = self.get_ai_color().expect("No AI found");
        let game_uuid = self.game_uuid.clone();

        let clients = Arc::clone(&self.clients);
        let board_clone = Arc::clone(&self.board);
        let game_meta = Arc::clone(&self.game_meta);
        let spectators_clone = self.spectators.clone();

        spawn(async move {
            let mut board_guard = board_clone.lock().await;

            while board_guard.get_turn() == ai_side && !board_guard.is_game_over() {
                let mut ai = AI {
                    side: ai_side,
                    board: board_guard.clone(),
                };
                if let Some(new_board) = ai.make_minmax_move() {
                    *board_guard = new_board;
                } else {
                    break;
                }

                let res: Response = Response::GameState {
                    game_uuid: game_uuid.clone(),
                    game_state: BoardFrontend::new(board_guard.clone()),
                };

                // TODO?: remove duplication
                clients
                    .lock()
                    .await
                    .iter_mut()
                    .filter(|(uuid, _)| spectators_clone.contains(uuid))
                    .for_each(|(_, client)| {
                        if let Some(sender) = &client.sender {
                            let _ = sender
                                .send(Ok(Message::text(serde_json::to_string(&res).unwrap())));
                        }
                    });
                task::yield_now().await;
            }

            let mut meta_guard = game_meta.lock().await;
            meta_guard.update_last_move_at();
            if board_guard.is_game_over() {
                meta_guard.promote_to_completed();
            }
        });
    }

    fn add_with_color_pref(
        &mut self,
        client_uuid: UserUUID,
        player: Player,
        color: ColorPreference,
    ) {
        match color {
            ColorPreference::AlwaysWhite => {
                self.player_white = Some((client_uuid, player));
            }
            ColorPreference::AlwaysBlack => {
                self.player_black = Some((client_uuid, player));
            }
            ColorPreference::Random => {
                if rand::thread_rng().gen() {
                    self.player_white = Some((client_uuid, player));
                } else {
                    self.player_black = Some((client_uuid, player));
                }
            }
        }
    }

    fn add_player(&mut self, client_uuid: UserUUID, player: Player) {
        match (&self.player_white, &self.player_black) {
            (None, None) => {
                self.add_with_color_pref(
                    client_uuid.clone(),
                    player,
                    self.game_configuration.side_selection,
                );
                if let PlayerType::Computer = self.game_configuration.opponent {
                    self.add_player(client_uuid, Player::new_computer());
                }
            }
            (None, Some(_)) => {
                self.add_with_color_pref(client_uuid, player, ColorPreference::AlwaysWhite);
            }
            (Some(_), None) => {
                self.add_with_color_pref(client_uuid, player, ColorPreference::AlwaysBlack);
            }
            (Some(_), Some(_)) => {
                panic!("Trying to add a new player to a game with two players.");
            }
        };
    }

    fn add_spectator(&mut self, client_uuid: UserUUID) {
        self.spectators.push(client_uuid)
    }
}
