use std::i32;
use std::time::{Duration, Instant};

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl SolutionAgent {
    #[inline(always)]
    fn other_player(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    #[inline(always)]
    fn eval_window(a: &Cell, b: &Cell, c: &Cell) -> i32 {
        let mut x = 0i32;
        let mut o = 0i32;
        for cell in [a, b, c] {
            match cell {
                Cell::X     => x += 1,
                Cell::O     => o += 1,
                Cell::Wall  => return 0,
                Cell::Empty => {}
            }
        }
        if x > 0 && o > 0 { return 0; }
        match (x, o) {
            (3, 0) => 500,
            (0, 3) => -500,
            (2, 0) => 20,
            (0, 2) => -20,
            (1, 0) => 2,
            (0, 1) => -2,
            _ => 0,
        }
    }

    fn heuristic(board: &Board) -> i32 {
        let cells = board.get_cells();
        let rows = cells.len();
        let cols = cells[0].len();
        let mut total: i32 = board.score() * 1000;

        for r in 0..rows {
            for c in 0..cols {
                if c + 2 < cols {
                    total += Self::eval_window(&cells[r][c], &cells[r][c+1], &cells[r][c+2]);
                }
                if r + 2 < rows {
                    total += Self::eval_window(&cells[r][c], &cells[r+1][c], &cells[r+2][c]);
                }
                if r + 2 < rows && c + 2 < cols {
                    total += Self::eval_window(&cells[r][c], &cells[r+1][c+1], &cells[r+2][c+2]);
                }
                if r + 2 < rows && c >= 2 {
                    total += Self::eval_window(&cells[r][c], &cells[r+1][c-1], &cells[r+2][c-2]);
                }
            }
        }
        total
    }

    fn count_empty(board: &Board) -> usize {
        board.get_cells().iter()
            .flat_map(|row| row.iter())
            .filter(|c| matches!(c, Cell::Empty))
            .count()
    }

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        max_depth: i32,
        mut alpha: i32,
        mut beta: i32,
        deadline: Instant,
        timed_out: &mut bool,
    ) -> (i32, usize, usize) {
        if *timed_out {
            return (0, 0, 0);
        }
        if depth <= 1 && Instant::now() >= deadline {
            *timed_out = true;
            return (0, 0, 0);
        }

        if board.game_over() {
            let score = board.score();
            return (score * 1000 + if score > 0 { -depth } else { depth }, 0, 0);
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
                for mv in &moves {
                    if *timed_out { break; }
                    board.apply_move(*mv, player);
                    let (score, _, _) = Self::minimax(
                        board, Self::other_player(player),
                        depth + 1, max_depth, alpha, beta, deadline, timed_out,
                    );
                    board.undo_move(*mv, player);

                    if score > best_score {
                        best_score = score;
                        best_move = *mv;
                    }
                    if best_score > alpha { alpha = best_score; }
                    if alpha >= beta { break; }
                }
                (best_score, best_move.0, best_move.1)
            }
            Player::O => {
                let mut best_score = i32::MAX;
                for mv in &moves {
                    if *timed_out { break; }
                    board.apply_move(*mv, player);
                    let (score, _, _) = Self::minimax(
                        board, Self::other_player(player),
                        depth + 1, max_depth, alpha, beta, deadline, timed_out,
                    );
                    board.undo_move(*mv, player);

                    if score < best_score {
                        best_score = score;
                        best_move = *mv;
                    }
                    if best_score < beta { beta = best_score; }
                    if alpha >= beta { break; }
                }
                (best_score, best_move.0, best_move.1)
            }
        }
    }
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let budget_ms = (time_limit as f64 * 0.78) as u64;
        let deadline = Instant::now() + Duration::from_millis(budget_ms);

        let empty = Self::count_empty(board);
        let hard_max_depth = match empty {
            18..=25 => 3,
            12..=17 => 4,
            8..=11  => 5,
            4..=7   => 6,
            _       => 10,
        };

        let fallback = board.moves().into_iter().next().unwrap_or((0, 0));
        let mut best_result = fallback;
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        for max_depth in 1..=hard_max_depth {
            if Instant::now() >= deadline { break; }

            let mut timed_out = false;
            let (score, r, c) = Self::minimax(
                board, player, 0, max_depth,
                i32::MIN, i32::MAX,
                deadline, &mut timed_out,
            );

            if !timed_out {
                best_score = score;
                best_result = (r, c);
            } else {
                break;
            }
        }

        (best_score, best_result.0, best_result.1)
    }
}