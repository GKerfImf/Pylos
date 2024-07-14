use std::ops::Range;

use log::{error, warn};

use super::{ball::Ball, player_side::PlayerSide};
use crate::board::{board_side::BoardSide, index::Index};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Copy)]
pub struct Move {
    from: Ball,
    to: Ball,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BoardState {
    pub nmove: u8,
    pub turn: PlayerSide,
    pub takeDownRule: u8,                // TODO: rename
    pub selectedBall: Option<Ball>,      // TODO?: delete
    pub selectedGhostBall: Option<Ball>, // TODO?: delete
    pub balls: Vec<Ball>,
}

impl BoardState {
    pub fn new() -> BoardState {
        let balls: Vec<Ball> = (0..5)
            .flat_map(|x| {
                (0..3)
                    .flat_map(|y| {
                        vec![
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

        BoardState {
            nmove: 1,
            turn: PlayerSide::White,
            takeDownRule: 0,
            selectedBall: None,
            selectedGhostBall: None,
            balls,
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardState {
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
        BoardState::find_parents(child)
            .into_iter()
            .any(|p| p == parent)
    }

    fn find_ball(&self, index: Index) -> Option<Ball> {
        self.balls
            .clone()
            .into_iter()
            .find(|ball| ball.index == index)
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
        BoardState::find_parents(index)
            .into_iter()
            .any(|i| !self.empty_index(i))
    }

    fn take_back_is_possible(&self, player: PlayerSide) -> bool {
        self.balls
            .clone()
            .into_iter()
            .filter(|ball| ball.index.b == BoardSide::Center)
            .filter(|ball| ball.player == player)
            .any(|ball| !self.parent_exists(ball.index))
    }

    fn has_ball_in_reserve(&self, player: PlayerSide) -> bool {
        let players_board_side = if player == PlayerSide::White {
            BoardSide::White
        } else {
            BoardSide::Black
        };

        self.balls
            .clone()
            .into_iter()
            .any(|ball| ball.index.b == players_board_side)
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
                    BoardState::find_children(index)
                        .into_iter()
                        .all(|index| !self.empty_index(index))
                })
                .filter(|&index| !BoardState::is_parent(ball.index, index))
                .map(|index| Ball { player, index })
                .collect()
        }
    }

    fn move_is_possible(&self, player: PlayerSide) -> bool {
        if self.has_ball_in_reserve(player) {
            return true;
        }

        self.balls
            .clone()
            .into_iter()
            .filter(|ball| ball.index.b == BoardSide::Center)
            .filter(|ball| ball.player == player)
            .filter(|ball| !self.parent_exists(ball.index))
            .any(|ball| !self.get_ghost_balls(ball).is_empty())
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
        BoardState::find_parents(ball.index)
            .into_iter()
            .map(BoardState::find_children)
            .any(|is| self.same_color_balls(is, self.get_turn()))
    }

    fn remove_ball(&mut self, ball: Ball) -> bool {
        if self.ball_exists(ball) {
            self.balls = self
                .balls
                .clone()
                .into_iter()
                .filter(|b| *b != ball)
                .collect();
            true
        } else {
            false
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

    pub fn get_turn(&self) -> PlayerSide {
        self.turn
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        if !self.player_color_matches_ball_color(mv) {
            warn!("Player is trying to move a ball of the opposite color");
            return false;
        }

        if !self.take_back_rule() {
            let Move { from, to } = mv;

            self.increase_move_number();
            self.remove_ball(from);
            self.add_ball(to);

            if self.square_is_formed(to) {
                self.bump_take_back_counter();
            } else if self.move_is_possible(!self.get_turn()) {
                self.pass_turn();
            }

            true
        } else if self.take_back_rule() && self.move_from_main_board_to_side_board(mv) {
            let Move { from, to } = mv;
            self.remove_ball(from);
            self.add_ball(to);

            self.decrease_take_back_counter();
            if self.take_back_rule() && self.take_back_is_possible(self.get_turn()) {
                return true;
            }

            self.reset_take_back_counter();
            if self.move_is_possible(!self.get_turn()) {
                self.pass_turn();
            }

            true
        } else {
            error!("This branch is supposed to be unreachable!");
            false
        }
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        self.balls
            .clone()
            .into_iter()
            .filter(|ball| ball.player == self.get_turn())
            .filter(|ball| !self.parent_exists(ball.index))
            .flat_map(|from| {
                self.get_ghost_balls(from)
                    .into_iter()
                    .map(move |to| Move { from, to })
            })
            .collect()
    }
}
