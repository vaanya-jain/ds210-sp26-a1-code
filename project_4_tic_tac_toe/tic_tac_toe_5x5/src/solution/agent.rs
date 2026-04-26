use std::i32;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
        let cells: &Vec<Vec<Cell>> = board.get_cells();
        let rows: usize = cells.len();
        let cols: usize = cells[0].len();

        let mut total: i32 = 0;

        fn eval_segment(a: &Cell, b: &Cell, c: &Cell) -> i32 {
            let mut x_count = 0;
            let mut o_count = 0;
            let mut empty_count = 0;

            for cell in [a, b, c] {
                match cell {
                    Cell::X => x_count += 1,
                    Cell::O => o_count += 1,
                    Cell::Empty => empty_count += 1,
                    Cell::Wall => return 0,
                }
            }

            if x_count > 0 && o_count > 0 {
                return 0;
            }

            match (x_count, o_count, empty_count) {
                (3, 0, 0) => 100,
                (2, 0, 1) => 12,
                (1, 0, 2) => 2,
                (0, 3, 0) => -100,
                (0, 2, 1) => -12,
                (0, 1, 2) => -2,
                _ => 0,
            }
        }

        // horizontal
        for r in 0..rows {
            for c in 0..=(cols - 3) {
                total += eval_segment(&cells[r][c], &cells[r][c + 1], &cells[r][c + 2]);
            }
        }

        // vertical
        for r in 0..=(rows - 3) {
            for c in 0..cols {
                total += eval_segment(&cells[r][c], &cells[r + 1][c], &cells[r + 2][c]);
            }
        }

        // diagonal down-right
        for r in 0..=(rows - 3) {
            for c in 0..=(cols - 3) {
                total += eval_segment(&cells[r][c], &cells[r + 1][c + 1], &cells[r + 2][c + 2]);
            }
        }

        // diagonal down-left
        for r in 0..=(rows - 3) {
            for c in 2..cols {
                total += eval_segment(&cells[r][c], &cells[r + 1][c - 1], &cells[r + 2][c - 2]);
            }
        }

        total
    }

    fn other_player(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn minimax(board: &mut Board, player: Player, depth: i32, max_depth: i32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
        if board.game_over() {
            let score = board.score();

            if score > 0 {
                return (100 - depth, 0, 0);
            } else if score < 0 {
                return (-100 + depth, 0, 0);
            } else {
                return (0, 0, 0);
            }
        }

        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }

        let moves = board.moves();

        if moves.is_empty() {
            return (Self::heuristic(board), 0, 0);
        }

        let mut best_move = moves[0];

        match player {
            Player::X => {
                let mut best_score = i32::MIN;

                for mv in moves {
                    board.apply_move(mv, player);

                    let (score, _, _) = Self::minimax(board, Self::other_player(player), depth + 1, max_depth, alpha, beta);

                    board.undo_move(mv, player);

                    if score > best_score {
                        best_score = score;
                        best_move = mv;
                    }

                    if best_score > alpha {
                        alpha = best_score;
                    }

                    if alpha >= beta {
                        break;
                    }
                }

                (best_score, best_move.0, best_move.1)
            }
            Player::O => {
                let mut best_score = i32::MAX;

                for mv in moves {
                    board.apply_move(mv, player);

                    let (score, _, _) = Self::minimax(board, Self::other_player(player), depth + 1, max_depth, alpha, beta);

                    board.undo_move(mv, player);

                    if score < best_score {
                        best_score = score;
                        best_move = mv;
                    }

                    if best_score < beta {
                        beta = best_score;
                    }

                    if alpha >= beta {
                        break;
                    }
                }

                (best_score, best_move.0, best_move.1)
            }
        }
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        let max_depth = 4;
        Self::minimax(board, player, 0, max_depth, i32::MIN, i32::MAX)
    }
}