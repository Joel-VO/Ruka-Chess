mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board, ChessMove};
use crate::search::alpha_beta::best_move;
use std::{io, time::Instant, time::Duration};
// use crate::search::search_improvements::quiescent_search::tactical_moves;
fn main() {
    println!("Enter fen string ");
    let max_time = Duration::new(1,0);//seconds and nano-seconds adjustments
    let mut fen:String = String::new();
    io::stdin().read_line(&mut fen).expect("Data not a string");
    //  4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24
    // rn3r1k/p2q4/bp2pp1p/3pP3/P2NRQ2/1Pb2NPP/5PB1/3R2K1 w - - 1 22
    // rnbqkbnr/pp1pppp1/8/2pP3p/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3
    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];//takes just the current player

        match Board::from_str(fen.as_str()){
        Ok(board) => {//checks condition to see if board is legal
            //checks whose turn it is currently and feeds to alpha beta
            let is_maximising = if piece_to_move == "b"{
                false
            }else{
                true
            };
            // iterative deepening code.
            let (mut best_mov,mut eval):(ChessMove, i32) = (ChessMove::default(), 0);

            let now = Instant::now();//starts the time.
            for depth_iterate in 6..100{//the timing logic can be fine-tuned a lot based on available time, position etc.
                let mut elapsed = now.elapsed();//checks if time constraint is passed.
                //the timing logic has to be changed to make sure live timing is possible so it takes only the specified amount of time.
                if elapsed<=max_time{
                    if let Some((mov, evaluation)) = best_move(&board, is_maximising, depth_iterate) {
                        (best_mov, eval) = (mov, evaluation);
                    } else {
                        println!("No moves available");
                        break;
                    }
                }else{
                    println!("max depth was: {depth_iterate}");
                    break
                }
            }
            println!("best move is {best_mov} with eval as {eval}");
            // let moves = tactical_moves(&board);
            // for mv in moves{
            //     println!("{mv}");
            // }
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
}
