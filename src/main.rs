mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;
fn main(){
    let fen = "4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24";
    match Board::from_str(fen){
        Ok(board) => {
            if let Some(mov) = best_move(&board, true, 6){
                println!("best move is {mov}");
            }else{
                println!("No moves available");
            }
        }
        Err(err) => {
            println!("error {err}");
        }
    }
}