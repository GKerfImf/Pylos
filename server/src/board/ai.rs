use super::{board_state::BoardState, player_side::PlayerSide};
use rand::Rng;

pub struct AI {
    pub side: PlayerSide,
    pub board: BoardState,
}

impl AI {
    pub fn new(side: PlayerSide) -> AI {
        AI {
            side,
            board: BoardState::new(),
        }
    }

    pub fn make_random_moves(&mut self) -> BoardState {
        while self.board.get_turn() == self.side {
            let moves = self.board.get_valid_moves();
            if moves.is_empty() {
                break;
            }

            let mut rng = rand::thread_rng();
            let mv = moves[rng.gen::<usize>() % moves.len()];

            self.board.make_move(mv);
        }

        self.board.clone()
    }

}
