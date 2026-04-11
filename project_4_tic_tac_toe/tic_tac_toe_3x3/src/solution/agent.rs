use std::i32;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        // If you want to make a recursive call to this solution, use
        // `SolutionAgent::solve(...)`
        if board.game_over() {
            return(board.score(), 0, 0);
        }

        let moves = board.moves();
        let mut best_score = match player {
            Player::X => i32::MIN,
            Player::O => i32::MAX,
        };

        let mut best_move = (0, 0);
    }
    for i in moves {
    let mut next_board = board.clone();
    next_board.apply_move(i, player);

    let next_player = match player {
    Player::X => Player::O,
    Player::O => Player::X,
    };

    let (score, _, _) = SolutionAgent::solve(&mut next_board, next_player, _time_limit);
    }
    
}
