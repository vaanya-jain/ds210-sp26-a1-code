use std::i32;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution.
pub struct SolutionAgent {}

impl SolutionAgent {
    fn heuristic(board: &Board) -> i32 {
        // simplest heuristic for incomplete boards
        board.score()
    }

    fn other_player(player: Player) -> Player {
        match player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn minimax(
        board: &mut Board,
        player: Player,
        depth: i32,
        max_depth: i32,
    ) -> (i32, usize, usize) {
        // true terminal state
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

        // step 2: stop early and use heuristic
        if depth == max_depth {
            return (Self::heuristic(board), 0, 0);
        }

        let moves = board.moves();

        // safety check
        if moves.is_empty() {
            return (Self::heuristic(board), 0, 0);
        }

        let mut best_move = moves[0];

        match player {
            Player::X => {
                let mut best_score = i32::MIN;

                for mv in moves {
                    board.apply_move(mv, player);

                    let (score, _, _) =
                        Self::minimax(board, Self::other_player(player), depth + 1, max_depth);

                    board.undo_move(mv, player);

                    if score > best_score {
                        best_score = score;
                        best_move = mv;
                    }
                }

                (best_score, best_move.0, best_move.1)
            }
            Player::O => {
                let mut best_score = i32::MAX;

                for mv in moves {
                    board.apply_move(mv, player);

                    let (score, _, _) =
                        Self::minimax(board, Self::other_player(player), depth + 1, max_depth);

                    board.undo_move(mv, player);

                    if score < best_score {
                        best_score = score;
                        best_move = mv;
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
        Self::minimax(board, player, 0, max_depth)
    }
}