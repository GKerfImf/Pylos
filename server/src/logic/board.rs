use std::fmt;
use std::ops::Range;

use super::{amove::Move, ball::Ball, player_side::PlayerSide};
use crate::logic::{board_side::BoardSide, index::Index};

fn cross(xs: Range<i8>, ys: Range<i8>) -> impl Iterator<Item = (i8, i8)> {
    ys.flat_map(move |y| xs.clone().map(move |x| (x, y)))
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BoardFrontend {
    pub nmove: u8,
    pub turn: PlayerSide,
    pub takeDownRule: u8, // TODO: rename
    pub balls: Vec<Ball>,
    pub winner: Option<PlayerSide>,
}

impl BoardFrontend {
    pub fn new(board: Board) -> Self {
        BoardFrontend {
            nmove: board.move_number,
            turn: board.turn,
            takeDownRule: board.take_back,
            balls: [
                Board::all_indices(BoardSide::White),
                Board::all_indices(BoardSide::Black),
                Board::all_indices(BoardSide::Center),
            ]
            .concat()
            .into_iter()
            .filter_map(|index| board.get_ball(index))
            .collect(),
            winner: board.winner,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    move_number: u8,
    turn: PlayerSide,
    take_back: u8,

    white_reserve: Vec<bool>,
    black_reserve: Vec<bool>,
    main_board: Vec<Option<PlayerSide>>,

    winner: Option<PlayerSide>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            move_number: 0,
            turn: PlayerSide::White,
            take_back: 0,

            white_reserve: vec![true; 5 * 3],
            black_reserve: vec![true; 5 * 3],
            main_board: vec![None; 4 * 4 * 4],

            winner: None,
        }
    }

    #[rustfmt::skip]
    fn get(&self, index: Index) -> Option<PlayerSide> {
        match index {
            Index { b: BoardSide::White, x, y, z: _ } => {
                if self.white_reserve[(x * 3 + y) as usize] {
                    Some(PlayerSide::White)
                } else {
                    None
                }
            }
            Index { b: BoardSide::Black, x, y, z: _ } => {
                if self.black_reserve[(x * 3 + y) as usize] {
                    Some(PlayerSide::Black)
                } else {
                    None
                }
            }
            Index { b: BoardSide::Center, x, y, z } => {
                self.main_board[(x * 16 + y * 4 + z) as usize]
            }
        }
    }

    fn take_back_rule(&self) -> bool {
        self.take_back > 0
    }

    pub fn get_winner(&self) -> Option<PlayerSide> {
        self.winner
    }

    pub fn get_turn(&self) -> PlayerSide {
        self.turn
    }

    #[rustfmt::skip]
    fn remove_ball(&mut self, ball: Ball) -> Result<(), &'static str> {
        if self.ball_exists(ball) {
            match ball.index {
                Index { b: BoardSide::White, x, y, z: _ } => {
                    self.white_reserve[(x * 3 + y) as usize] = false;
                }
                Index { b: BoardSide::Black, x, y, z: _ } => {
                    self.black_reserve[(x * 3 + y) as usize] = false;
                }
                Index { b: BoardSide::Center, x, y, z } => {
                    self.main_board[(x * 16 + y * 4 + z) as usize] = None;
                }
            }
            Ok(())
        } else {
            Err("[remove_ball]: ball does not exist")
        }
    }

    #[rustfmt::skip]
    fn add_ball(&mut self, ball: Ball) -> Result<(), &'static str> {
        if self.ball_exists(ball) {
            Err("[add_ball]: ball already exists")
        } else {
            match ball.index {
                Index { b: BoardSide::White, x, y, z: _ } => {
                    self.white_reserve[(x * 3 + y) as usize] = true;
                }
                Index { b: BoardSide::Black, x, y, z: _ } => {
                    self.black_reserve[(x * 3 + y) as usize] = true;
                }
                Index { b: BoardSide::Center, x, y, z } => {
                    self.main_board[(x * 16 + y * 4 + z) as usize] = Some(ball.player);
                }
            }
            Ok(())
        }
    }

    fn increase_move_number(&mut self) -> Result<(), &'static str> {
        self.move_number += 1;
        Ok(())
    }

    pub fn get_move_number(&self) -> u8 {
        self.move_number
    }

    fn pass_turn(&mut self) -> Result<(), &'static str> {
        self.turn = !self.get_turn();
        Ok(())
    }

    fn bump_take_back_counter(&mut self) -> Result<(), &'static str> {
        if self.take_back > 0 {
            return Err("[bump_take_back_counter] bump the counter twice should not be possible");
        }

        self.take_back += 2;
        Ok(())
    }

    fn decrease_take_back_counter(&mut self) -> Result<(), &'static str> {
        if self.take_back == 0 {
            return Err("[decrease_take_back_counter] counter cannot be negative");
        }

        self.take_back -= 1;
        Ok(())
    }

    fn reset_take_back_counter(&mut self) -> Result<(), &'static str> {
        self.take_back = 0;
        Ok(())
    }
}

impl Board {
    fn child_indices(index: Index) -> Vec<Index> {
        if index.b != BoardSide::Center {
            return vec![];
        }

        let Index { b, x, y, z } = index;

        vec![
            Index { b, x, y, z: z + 1 },
            Index {
                b,
                x: x - 1,
                y,
                z: z + 1,
            },
            Index {
                b,
                x,
                y: y - 1,
                z: z + 1,
            },
            Index {
                b,
                x: x - 1,
                y: y - 1,
                z: z + 1,
            },
        ]
        .into_iter()
        .filter(|i| 0 <= i.x && i.x < 3 - z)
        .filter(|i| 0 <= i.y && i.y < 3 - z)
        .filter(|i| 0 <= i.z && i.z < 4)
        .collect()
    }

    fn parent_indices(index: Index) -> Vec<Index> {
        if index.b != BoardSide::Center {
            return vec![];
        }

        let Index { b, x, y, z } = index;

        vec![
            Index { b, x, y, z: z - 1 },
            Index {
                b,
                x: x + 1,
                y,
                z: z - 1,
            },
            Index {
                b,
                x,
                y: y + 1,
                z: z - 1,
            },
            Index {
                b,
                x: x + 1,
                y: y + 1,
                z: z - 1,
            },
        ]
        .into_iter()
        .filter(|i| 0 <= i.x && i.x <= 4 - z)
        .filter(|i| 0 <= i.y && i.y <= 4 - z)
        .filter(|i| 0 <= i.z && i.z < 4)
        .collect()
    }

    fn is_child_index(index: Index, child: Index) -> bool {
        Board::child_indices(index).iter().any(|&i| child == i)
    }

    fn all_indices(board_side: BoardSide) -> Vec<Index> {
        match board_side {
            BoardSide::Center => [4, 3, 2, 1]
                .iter()
                .flat_map(|&z| cross(0..z, 0..z).map(move |(x, y)| (x, y, 4 - z)))
                .map(|(x, y, z)| Index {
                    b: board_side,
                    x,
                    y,
                    z,
                })
                .collect(),
            _ => cross(0..5, 0..3)
                .map(|(x, y)| Index {
                    b: board_side,
                    x,
                    y,
                    z: 0,
                })
                .collect(),
        }
    }

    fn player_side_to_board_side(player_side: PlayerSide) -> BoardSide {
        if player_side == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        }
    }
}

impl Board {
    fn get_ball(&self, index: Index) -> Option<Ball> {
        self.get(index).map(|player| Ball { player, index })
    }

    fn is_index_empty(&self, index: Index) -> bool {
        self.get(index).is_none()
    }

    fn is_index_full(&self, index: Index) -> bool {
        self.get(index).is_some()
    }

    fn child_exists(&self, index: Index) -> bool {
        Board::child_indices(index)
            .iter()
            .any(|&i| self.is_index_full(i))
    }

    fn all_parent_exist(&self, index: Index) -> bool {
        Board::parent_indices(index)
            .iter()
            .all(|&i| self.is_index_full(i))
    }

    fn no_children_exist(&self, index: Index) -> bool {
        Board::child_indices(index)
            .iter()
            .all(|&i| self.is_index_empty(i))
    }

    fn ball_exists(&self, ball: Ball) -> bool {
        if let Some(new_ball) = self.get_ball(ball.index) {
            new_ball == ball
        } else {
            false
        }
    }

    fn take_back_is_possible(&self, player: PlayerSide) -> bool {
        Board::all_indices(BoardSide::Center)
            .iter()
            .filter_map(|&index| self.get_ball(index))
            .filter(|ball| ball.player == player)
            .any(|ball| !self.child_exists(ball.index))
    }

    pub fn number_of_balls_in_reserve(&self, player: PlayerSide) -> usize {
        let board_side = Board::player_side_to_board_side(player);

        Board::all_indices(board_side)
            .iter()
            .filter(|&&index| self.is_index_full(index))
            .collect::<Vec<_>>()
            .len()
    }

    fn has_ball_in_reserve(&self, player: PlayerSide) -> bool {
        let board_side = Board::player_side_to_board_side(player);

        Board::all_indices(board_side)
            .iter()
            .any(|&index| self.is_index_full(index))
    }

    pub fn move_up_is_possible(&self, player: PlayerSide) -> bool {
        let free_indices = Board::all_indices(BoardSide::Center)
            .into_iter()
            .filter(|&index| self.is_index_empty(index))
            .filter(|&index| self.all_parent_exist(index))
            .collect::<Vec<_>>();

        Board::all_indices(BoardSide::Center)
            .iter()
            .filter(|&&index| self.is_index_full(index))
            .map(|&index| Ball { player, index })
            .filter(|&ball| self.ball_exists(ball))
            .filter(|&ball| self.no_children_exist(ball.index))
            .any(|ball| {
                free_indices
                    .iter()
                    .filter(|&index| ball.index.z < index.z)
                    .any(|&index| !Board::is_child_index(ball.index, index))
            })
    }

    fn move_is_possible(&self, player: PlayerSide) -> bool {
        if self.take_back_rule() {
            return self.take_back_is_possible(player);
        }

        if self.has_ball_in_reserve(player) {
            return true;
        }

        self.move_up_is_possible(player)
    }

    fn player_color_matches_ball_color(&self, mv: Move) -> bool {
        self.get_turn() == mv.from.player && self.get_turn() == mv.to.player
    }

    fn move_from_main_board_to_side_board(&self, mv: Move) -> bool {
        let players_board_side = Board::player_side_to_board_side(self.get_turn());
        let Move { from, to } = mv;
        from.index.b == BoardSide::Center && to.index.b == players_board_side
    }

    fn same_color_balls(&self, indices: &[Index], color: PlayerSide) -> bool {
        indices.iter().map(|&i| self.get_ball(i)).all(|ob| {
            if let Some(ball) = ob {
                ball.player == color
            } else {
                false
            }
        })
    }

    fn square_is_formed(&self, ball: Ball) -> bool {
        Board::child_indices(ball.index)
            .into_iter()
            .map(Board::parent_indices)
            .any(|is| self.same_color_balls(&is, self.get_turn()))
    }

    pub fn is_game_over(&self) -> bool {
        self.get_winner().is_some()
    }

    fn validate_move(&mut self, mv: Move) -> Result<(), &'static str> {
        if !self.player_color_matches_ball_color(mv) {
            return Err("Player is trying to move a ball of the opposite color");
        }

        if self.is_game_over() {
            return Err("The game is over");
        }

        if !self.ball_exists(mv.from) {
            return Err("The ball does not exist");
        }

        if !self.all_parent_exist(mv.to.index) {
            return Err("Not all parent exist");
        }

        if !self.is_index_empty(mv.to.index) {
            return Err("The ball already exists");
        }

        Ok(())
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), &'static str> {
        self.validate_move(mv)?;

        if mv.to.index.z == 3 {
            self.winner = Some(mv.to.player);
        }

        if !self.take_back_rule() {
            let Move { from, to } = mv;

            let _ = self.increase_move_number(); // TODO: State change happens here.
            let _ = self.remove_ball(from); // TODO: If (e.g.) this change is rejected, the state will be inconsistent.
            let _ = self.add_ball(to);

            if self.square_is_formed(to) {
                let _ = self.bump_take_back_counter();
            } else if self.move_is_possible(!self.get_turn()) {
                let _ = self.pass_turn();
            }

            Ok(())
        } else if self.take_back_rule() && self.move_from_main_board_to_side_board(mv) {
            let Move { from, to } = mv;
            let _ = self.remove_ball(from);
            let _ = self.add_ball(to);

            let _ = self.decrease_take_back_counter();
            if self.take_back_rule() && self.take_back_is_possible(self.get_turn()) {
                return Ok(());
            }

            let _ = self.reset_take_back_counter();
            if self.move_is_possible(!self.get_turn()) {
                let _ = self.pass_turn();
            }

            Ok(())
        } else {
            panic!("This branch is supposed to be unreachable!")
        }
    }

    // TODO: clean up
    pub fn get_valid_moves(&self) -> Vec<Move> {
        let board_side = Board::player_side_to_board_side(self.get_turn());

        if self.take_back_rule() {
            let to = Board::all_indices(board_side)
                .into_iter()
                .find(|&index| self.is_index_empty(index))
                .map(|index| Ball {
                    player: self.get_turn(),
                    index,
                })
                .unwrap();

            Board::all_indices(BoardSide::Center)
                .into_iter()
                .filter_map(|index| self.get_ball(index))
                .filter(|ball| ball.player == self.get_turn())
                .filter(|ball| !self.child_exists(ball.index))
                .map(|ball| Move { from: ball, to })
                .collect()
        } else {
            let res_to_center = {
                let from = Board::all_indices(board_side)
                    .into_iter()
                    .find(|&index| self.is_index_full(index))
                    .map(|index| Ball {
                        player: self.get_turn(),
                        index,
                    });

                if let Some(from) = from {
                    Board::all_indices(BoardSide::Center)
                        .into_iter()
                        .filter(|&index| self.is_index_empty(index))
                        .filter(|&index| self.all_parent_exist(index))
                        .map(|index| Ball {
                            player: self.get_turn(),
                            index,
                        })
                        .map(|to| Move { from, to })
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            };
            let center_to_center = {
                let free_indices = Board::all_indices(BoardSide::Center)
                    .into_iter()
                    .filter(|&index| self.is_index_empty(index))
                    .filter(|&index| self.all_parent_exist(index))
                    .collect::<Vec<_>>();

                let movable_balls = Board::all_indices(BoardSide::Center)
                    .iter()
                    .filter(|&&index| self.is_index_full(index))
                    .map(|&index| Ball {
                        player: self.get_turn(),
                        index,
                    })
                    .filter(|&ball| self.ball_exists(ball))
                    .filter(|&ball| self.no_children_exist(ball.index))
                    .collect::<Vec<_>>();

                let res = movable_balls
                    .into_iter()
                    .flat_map(|from| {
                        free_indices
                            .iter()
                            .filter(move |&index| from.index.z < index.z)
                            .filter(move |&&index| !Board::is_child_index(from.index, index))
                            .map(|&index| Ball {
                                player: self.get_turn(),
                                index,
                            })
                            .map(move |to| Move { from, to })
                    })
                    .collect::<Vec<_>>();

                res
            };

            [res_to_center, center_to_center].concat()
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        writeln!(f).ok();
        for y in 0..4 {
            for z in 0..(4 - y) {
                write!(f, "[ ").ok();
                for x in 0..(4 - z) {
                    match self.get_ball(Index {
                        b: BoardSide::Center,
                        x,
                        y,
                        z,
                    }) {
                        Some(Ball {
                            player: PlayerSide::White,
                            index: _,
                        }) => {
                            write!(f, "◯ ").unwrap();
                        }
                        Some(Ball {
                            player: PlayerSide::Black,
                            index: _,
                        }) => {
                            write!(f, "● ").unwrap();
                        }
                        None => {
                            write!(f, "⋅ ").ok();
                        }
                    }
                }
                write!(f, "]  ").ok();
            }
            writeln!(f).ok();
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_balls_same_place() {
        let mut board = Board::new();
        assert!(board.make_move(Move::new_wrc((0, 0), (0, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 0), (0, 0, 0))).is_err());
    }

    #[test]
    fn no_children_move() {
        let mut board = Board::new();
        assert!(board.make_move(Move::new_wrc((0, 0), (0, 0, 3))).is_err());
    }

    #[test]
    fn no_wrong_color_move() {
        let mut board = Board::new();
        assert!(board.make_move(Move::new_wrc((0, 0), (0, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((1, 1), (1, 0, 0))).is_err());
    }

    #[test]
    fn square_formed() {
        let mut board = Board::new();

        assert!(board.make_move(Move::new_wrc((0, 0), (0, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 0), (3, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((1, 0), (0, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((1, 0), (3, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((2, 0), (1, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((2, 0), (2, 3, 0))).is_ok());
        assert!(!board.take_back_rule());

        assert!(board.make_move(Move::new_wrc((3, 0), (1, 1, 0))).is_ok());
        assert!(board.square_is_formed(Ball::new_wc(1, 1, 0)));
        assert!(board.take_back_rule());
    }

    #[test]
    fn skip_take_back() {
        let mut board = Board::new();

        assert!(board.make_move(Move::new_wrc((0, 0), (0, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 0), (1, 2, 0))).is_ok());

        assert!(board.make_move(Move::new_wrc((1, 0), (1, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((1, 0), (1, 1, 0))).is_ok());

        assert!(board.make_move(Move::new_wrc((2, 0), (2, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((2, 0), (2, 1, 0))).is_ok());

        assert!(board.make_move(Move::new_wrc((3, 0), (1, 0, 1))).is_ok());
        assert!(board.make_move(Move::new_brc((3, 0), (0, 1, 0))).is_ok());

        assert!(board.make_move(Move::new_wrc((4, 0), (0, 1, 1))).is_ok());
        assert!(board.make_move(Move::new_brc((4, 0), (2, 2, 0))).is_ok());

        // [ ⋅ ◯ ◯ ⋅ ]  [ ⋅ ◯ ⋅ ]  [ ⋅ ⋅ ]  [ ⋅ ]
        // [ ● ● ● ⋅ ]  [ ◯ ⋅ ⋅ ]  [ ⋅ ⋅ ]
        // [ ◯ ● ● ⋅ ]  [ ⋅ ⋅ ⋅ ]
        // [ ⋅ ⋅ ⋅ ⋅ ]

        assert!(board.square_is_formed(Ball::new_bc(2, 2, 0)));
        assert!(board.take_back_rule());
        assert!(board.take_back_is_possible(PlayerSide::Black));
        assert_eq!(board.take_back, 2);

        assert!(board.make_move(Move::new_bcr((2, 2, 0), (4, 0))).is_ok());

        // [ ⋅ ◯ ◯ ⋅ ]  [ ⋅ ◯ ⋅ ]  [ ⋅ ⋅ ]  [ ⋅ ]
        // [ ● ● ● ⋅ ]  [ ◯ ⋅ ⋅ ]  [ ⋅ ⋅ ]
        // [ ◯ ● ⋅ ⋅ ]  [ ⋅ ⋅ ⋅ ]
        // [ ⋅ ⋅ ⋅ ⋅ ]

        assert!(!board.take_back_rule());
        assert!(!board.take_back_is_possible(PlayerSide::Black));
        assert_eq!(board.take_back, 0);
    }

    #[test]
    fn skip_moves() {
        let mut board = Board::new();

        assert!(board.make_move(Move::new_wrc((0, 0), (2, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 0), (1, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((1, 0), (2, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((1, 0), (0, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((2, 0), (0, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((2, 0), (1, 2, 0))).is_ok());

        assert!(board.make_move(Move::new_wcc((0, 1, 0), (1, 1, 1))).is_ok());
        assert!(!board.ball_exists(Ball::new_wc(0, 1, 0)));

        assert!(board.make_move(Move::new_brc((3, 0), (1, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((3, 0), (0, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((4, 0), (2, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_wcc((0, 1, 0), (1, 2, 1))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 1), (3, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((4, 0), (0, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((1, 1), (0, 2, 1))).is_ok());
        assert!(board.make_move(Move::new_wrc((0, 1), (0, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((2, 1), (0, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((1, 1), (1, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_bcc((3, 3, 0), (0, 1, 1))).is_ok());
        assert!(board.make_move(Move::new_wrc((2, 1), (3, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((3, 1), (3, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((3, 1), (3, 2, 0))).is_ok());

        assert!(board.take_back_is_possible(PlayerSide::White));
        assert!(board.make_move(Move::new_wcr((3, 1, 0), (2, 1))).is_ok());
        assert!(board.make_move(Move::new_wcr((3, 2, 0), (3, 1))).is_ok());

        assert!(board.make_move(Move::new_brc((4, 1), (3, 1, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((2, 1), (3, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((0, 2), (3, 3, 0))).is_ok());
        assert!(board.make_move(Move::new_wcc((3, 2, 0), (0, 0, 1))).is_ok());
        assert!(board.make_move(Move::new_bcc((3, 0, 0), (0, 1, 2))).is_ok());
        assert!(board.make_move(Move::new_wrc((3, 1), (2, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_brc((1, 2), (3, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_wcc((1, 0, 0), (2, 0, 1))).is_ok());
        assert!(board.make_move(Move::new_brc((2, 2), (1, 0, 0))).is_ok());
        assert!(board.make_move(Move::new_wrc((4, 1), (1, 0, 1))).is_ok());
        assert!(board.make_move(Move::new_brc((3, 2), (3, 2, 0))).is_ok());
        assert!(board.make_move(Move::new_wcc((2, 0, 1), (0, 0, 2))).is_ok());
        assert!(board.make_move(Move::new_brc((4, 2), (2, 1, 1))).is_ok());
        assert!(board.make_move(Move::new_wrc((0, 2), (2, 2, 1))).is_ok());
        assert!(board.make_move(Move::new_bcc((3, 0, 0), (1, 1, 2))).is_ok());
        assert!(board.make_move(Move::new_wrc((1, 2), (3, 0, 0))).is_ok());

        assert!(!board.move_is_possible(PlayerSide::Black));
        assert!(board.make_move(Move::new_wrc((2, 2), (2, 0, 1))).is_ok());
        assert!(!board.move_is_possible(PlayerSide::Black));
        assert!(board.make_move(Move::new_wrc((3, 2), (1, 0, 2))).is_ok());
        assert!(!board.move_is_possible(PlayerSide::Black));
        assert!(board.make_move(Move::new_wrc((4, 2), (0, 0, 3))).is_ok());
        assert!(board.is_game_over());

        // [ ● ● ◯ ◯ ]  [ ◯ ◯ ◯ ]  [ ◯ ◯ ]  [ ◯ ]
        // [ ◯ ● ◯ ● ]  [ ● ◯ ● ]  [ ● ● ]
        // [ ● ● ◯ ● ]  [ ● ◯ ◯ ]
        // [ ◯ ● ● ● ]
    }
}
