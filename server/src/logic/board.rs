use std::fmt;
use std::ops::Range;

use super::{amove::Move, ball::Ball, player_side::PlayerSide};
use crate::logic::{board_side::BoardSide, index::Index};

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Board {
    pub nmove: u8,
    pub turn: PlayerSide,
    pub takeDownRule: u8, // TODO: rename
    pub balls: Vec<Ball>,
    pub winner: Option<PlayerSide>,
}

impl Board {
    pub fn new() -> Board {
        let balls: Vec<Ball> = (0..5)
            .flat_map(|x| {
                (0..3)
                    .flat_map(|y| {
                        vec![
                            // TODO: use Ball::new()
                            Ball {
                                player: PlayerSide::White,
                                index: Index {
                                    b: BoardSide::White,
                                    x,
                                    y,
                                    z: 0,
                                },
                            },
                            Ball {
                                player: PlayerSide::Black,
                                index: Index {
                                    b: BoardSide::Black,
                                    x,
                                    y,
                                    z: 0,
                                },
                            },
                        ]
                    })
                    .collect::<Vec<Ball>>()
            })
            .collect();

        Board {
            nmove: 1,
            turn: PlayerSide::White,
            takeDownRule: 0,
            balls,
            winner: None,
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
                    match self.find_ball(Index {
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

impl Board {
    // TODO?: swap parents and children terminology
    fn find_parents(index: Index) -> Vec<Index> {
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

    fn find_children(index: Index) -> Vec<Index> {
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

    fn is_parent(child: Index, parent: Index) -> bool {
        Board::find_parents(child).into_iter().any(|p| p == parent)
    }

    fn find_ball(&self, index: Index) -> Option<Ball> {
        self.balls.iter().find(|&ball| ball.index == index).copied()
    }

    fn empty_index(&self, index: Index) -> bool {
        self.find_ball(index).is_none()
    }

    fn ball_exists(&self, ball: Ball) -> bool {
        let index = ball.index;
        if let Some(new_ball) = self.find_ball(index) {
            new_ball == ball
        } else {
            false
        }
    }

    fn parent_exists(&self, index: Index) -> bool {
        Board::find_parents(index)
            .into_iter()
            .any(|i| !self.empty_index(i))
    }

    fn all_children_exist(&self, index: Index) -> bool {
        Board::find_children(index)
            .into_iter()
            .all(|i| !self.empty_index(i))
    }

    fn take_back_is_possible(&self, player: PlayerSide) -> bool {
        self.balls
            .iter()
            .filter(|ball| ball.index.b == BoardSide::Center)
            .filter(|ball| ball.player == player)
            .any(|ball| !self.parent_exists(ball.index))
    }

    pub fn number_of_balls_in_reserve(&self, player: PlayerSide) -> usize {
        let players_board_side = if player == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        };

        self.balls
            .iter()
            .filter(|ball| ball.index.b == players_board_side)
            .collect::<Vec<_>>()
            .len()
    }

    fn has_ball_in_reserve(&self, player: PlayerSide) -> bool {
        self.number_of_balls_in_reserve(player) > 0
    }

    fn get_ghost_balls(&self, ball: Ball) -> Vec<Ball> {
        fn cross(xs: Range<i8>, ys: Range<i8>) -> impl Iterator<Item = (i8, i8)> {
            ys.flat_map(move |y| xs.clone().map(move |x| (x, y)))
        }

        let player = ball.player;
        let board_side = if player == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        };

        if self.take_back_rule() {
            cross(0..5, 0..3)
                .map(|(x, y)| Index {
                    b: board_side,
                    x,
                    y,
                    z: 0,
                })
                .filter(|&index| self.find_ball(index).is_none())
                .map(|index| Ball { player, index })
                .collect()
        } else {
            let indices = vec![4, 3, 2, 1]
                .into_iter()
                .filter(|&i| i < 4 - ball.index.z || ball.index.b != BoardSide::Center)
                .flat_map(|z| cross(0..z, 0..z).map(move |(x, y)| vec![x, y, 4 - z]));

            indices
                .map(|i| Index {
                    b: BoardSide::Center,
                    x: i[0],
                    y: i[1],
                    z: i[2],
                })
                .filter(|&index| self.empty_index(index))
                .filter(|&index| {
                    Board::find_children(index)
                        .into_iter()
                        .all(|index| !self.empty_index(index))
                })
                .filter(|&index| !Board::is_parent(ball.index, index))
                .map(|index| Ball { player, index })
                .collect()
        }
    }

    fn move_is_possible(&self, player: PlayerSide) -> bool {
        if self.has_ball_in_reserve(player) {
            return true;
        }

        self.balls
            .iter()
            .filter(|ball| ball.index.b == BoardSide::Center)
            .filter(|ball| ball.player == player)
            .filter(|ball| !self.parent_exists(ball.index))
            .any(|&ball| !self.get_ghost_balls(ball).is_empty())
    }

    fn player_color_matches_ball_color(&self, mv: Move) -> bool {
        self.get_turn() == mv.from.player && self.get_turn() == mv.to.player
    }

    fn take_back_rule(&self) -> bool {
        self.takeDownRule > 0
    }

    fn move_from_main_board_to_side_board(&self, mv: Move) -> bool {
        let players_board_side = if self.get_turn() == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        };
        let Move { from, to } = mv;
        from.index.b == BoardSide::Center && to.index.b == players_board_side
    }

    fn same_color_balls(&self, indices: Vec<Index>, color: PlayerSide) -> bool {
        indices.into_iter().map(|i| self.find_ball(i)).all(|ob| {
            if let Some(ball) = ob {
                ball.player == color
            } else {
                false
            }
        })
    }

    fn square_is_formed(&self, ball: Ball) -> bool {
        Board::find_parents(ball.index)
            .into_iter()
            .map(Board::find_children)
            .any(|is| self.same_color_balls(is, self.get_turn()))
    }

    fn remove_ball(&mut self, ball: Ball) -> Result<(), &'static str> {
        if self.ball_exists(ball) {
            self.balls.retain(|&b| b != ball);
            Ok(())
        } else {
            Err("[remove_ball]: ball does not exist")
        }
    }

    fn add_ball(&mut self, ball: Ball) -> bool {
        if self.ball_exists(ball) {
            false
        } else {
            self.balls.insert(0, ball);
            true
        }
    }

    fn increase_move_number(&mut self) -> bool {
        self.nmove += 1;
        true
    }

    fn pass_turn(&mut self) -> bool {
        self.turn = !self.get_turn();
        true
    }

    fn bump_take_back_counter(&mut self) -> bool {
        if self.takeDownRule > 0 {
            return false;
        }

        self.takeDownRule += 2;
        true
    }

    fn decrease_take_back_counter(&mut self) -> bool {
        if self.takeDownRule == 0 {
            return false;
        }

        self.takeDownRule -= 1;
        true
    }

    fn reset_take_back_counter(&mut self) -> bool {
        self.takeDownRule = 0;
        true
    }

    pub fn get_winner(&self) -> Option<PlayerSide> {
        self.winner
    }

    pub fn get_turn(&self) -> PlayerSide {
        self.turn
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

        if !self.all_children_exist(mv.to.index) {
            return Err("Not all children exist");
        }

        if !self.empty_index(mv.to.index) {
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

            self.increase_move_number(); // TODO: State change happens here.
            let _ = self.remove_ball(from); // TODO: If (e.g.) this change is rejected, the state will be inconsistent.
            self.add_ball(to);

            if self.square_is_formed(to) {
                self.bump_take_back_counter();
            } else if self.move_is_possible(!self.get_turn()) {
                self.pass_turn();
            }

            Ok(())
        } else if self.take_back_rule() && self.move_from_main_board_to_side_board(mv) {
            let Move { from, to } = mv;
            let _ = self.remove_ball(from);
            self.add_ball(to);

            self.decrease_take_back_counter();
            if self.take_back_rule() && self.take_back_is_possible(self.get_turn()) {
                return Ok(());
            }

            self.reset_take_back_counter();
            if self.move_is_possible(!self.get_turn()) {
                self.pass_turn();
            }

            Ok(())
        } else {
            panic!("This branch is supposed to be unreachable!")
        }
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        // TODO: separate function
        let board_side = if self.get_turn() == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        };

        let to_res: Vec<Move> = self
            .balls
            .iter()
            .find(|ball| ball.index.b == board_side)
            .into_iter()
            .flat_map(|&from| {
                self.get_ghost_balls(from)
                    .into_iter()
                    .map(move |to| Move { from, to })
            })
            .collect();

        let from_res: Vec<Move> = self
            .balls
            .iter()
            .filter(|ball| ball.player == self.get_turn())
            .filter(|ball| ball.index.b == BoardSide::Center)
            .filter(|ball| !self.parent_exists(ball.index))
            .flat_map(|&from| {
                self.get_ghost_balls(from)
                    .into_iter()
                    .map(move |to| Move { from, to })
            })
            .collect();

        if self.take_back_rule() {
            from_res
        } else {
            [to_res, from_res].concat()
        }
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
        assert_eq!(board.takeDownRule, 2);

        assert!(board.make_move(Move::new_bcr((2, 2, 0), (4, 0))).is_ok());

        // [ ⋅ ◯ ◯ ⋅ ]  [ ⋅ ◯ ⋅ ]  [ ⋅ ⋅ ]  [ ⋅ ]
        // [ ● ● ● ⋅ ]  [ ◯ ⋅ ⋅ ]  [ ⋅ ⋅ ]
        // [ ◯ ● ⋅ ⋅ ]  [ ⋅ ⋅ ⋅ ]
        // [ ⋅ ⋅ ⋅ ⋅ ]

        assert!(!board.take_back_rule());
        assert!(!board.take_back_is_possible(PlayerSide::Black));
        assert_eq!(board.takeDownRule, 0);
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
