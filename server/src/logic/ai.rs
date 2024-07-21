use crate::logic::amove::Move;

use super::{board::Board, player_side::PlayerSide};

use log::info;
use rand::Rng;
use std::{collections::HashMap, time::Instant};

pub struct AI {
    pub side: PlayerSide,
    pub board: Board,
}

fn average<I>(iter: I) -> Option<f64>
where
    I: Iterator,
    I::Item: Into<f64>,
{
    let mut count = 0;
    let mut sum = 0.0;

    for item in iter {
        sum += item.into();
        count += 1;
    }

    if count > 0 {
        Some(sum / count as f64)
    } else {
        None
    }
}

fn bool_to_int(b: bool) -> i32 {
    if b {
        1
    } else {
        -1
    }
}

impl AI {
    pub fn new(side: PlayerSide) -> AI {
        AI {
            side,
            board: Board::new(),
        }
    }

    pub fn make_random_moves(&mut self) -> Board {
        while self.board.get_turn() == self.side {
            let moves = self.board.get_valid_moves();
            if moves.is_empty() {
                break;
            }

            let mut rng = rand::thread_rng();
            let mv = moves[rng.gen::<usize>() % moves.len()];

            let _ = self.board.make_move(mv);
        }

        self.board.clone()
    }

    pub fn minmax_moves(&mut self) -> Board {
        fn terminate_search(board: &Board) -> (i32, Option<Move>) {
            fn value(board: &Board) -> i32 {
                board.number_of_balls_in_reserve(board.get_turn()) as i32
                    - board.number_of_balls_in_reserve(!board.get_turn()) as i32
            }

            let moves = board.get_valid_moves();
            if moves.is_empty() {
                (value(board), None)
            } else {
                (value(board), Some(moves[0]))
            }
        }

        fn minmax(
            board: Board,
            fuel: i32,
            hash: &mut HashMap<Board, (i32, Option<Move>)>,
        ) -> (i32, Option<Move>) {
            if let Some(res) = hash.get(&board) {
                return *res;
            }

            if board.get_move_number() > 200 {
                return terminate_search(&board);
            }
            if fuel <= 0 {
                return terminate_search(&board);
            }

            if board.is_game_over() {
                if board.get_winner().unwrap() == board.get_turn() {
                    return (1000, None);
                } else {
                    return (-1000, None);
                }
            }

            let moves = board.get_valid_moves();

            let n: i32 = f32::powf(fuel as f32, 0.5).round() as i32;
            let n: i32 = if n < 25 { 1 } else { n };
            let n: i32 = n.min(moves.len() as i32);

            let res = moves
                .into_iter()
                .take(n as usize)
                .map(|mv| {
                    let mut new_board = board.clone();
                    let _ = new_board.make_move(mv);
                    let mult = bool_to_int(board.get_turn() == new_board.get_turn());

                    (new_board, mv, mult)
                })
                .map(|(board, mv, mult)| (minmax(board, fuel / n - 1, hash).0, mv, mult))
                .map(|(score, mv, mult)| (mult * score, Some(mv)))
                .collect::<Vec<_>>();

            let average_score = average(res.clone().into_iter().map(|(score, _)| score))
                .unwrap()
                .round() as i32;

            let best_move = res
                .clone()
                .into_iter()
                .max_by_key(|(score, _)| *score)
                .unwrap()
                .1;

            hash.insert(board, (average_score, best_move));

            (average_score, best_move)
        }

        let fuel = 1_000_000;
        let mut hash: HashMap<Board, (i32, Option<Move>)> = HashMap::new();

        while self.board.get_turn() == self.side && !self.board.is_game_over() {
            let start = Instant::now();
            let (score, omove) = minmax(self.board.clone(), fuel, &mut hash);
            let duration = start.elapsed();

            if omove.is_none() {
                break;
            }

            info!(
                "[minmax_moves, turn={:?}, mv={}, score={}, duration={:?}]",
                self.board.get_turn(),
                omove.unwrap().clone(),
                score,
                duration
            );
            let _ = self.board.make_move(omove.unwrap());
        }

        self.board.clone()
    }
}
