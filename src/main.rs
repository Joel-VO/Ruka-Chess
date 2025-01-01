mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;


fn main() {
    let fen = "2r5/1pB2Rbk/6pn/4n1q1/P3B3/1P5P/6P1/2Q1R2K w - - 1 34";//mate in 11 for white
    //checks whose turn it is currently and feeds to alpha beta
    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];
        match Board::from_str(fen) {
        Ok(board) => {
            let is_maximising = if piece_to_move == "b"{
                false
            }else{
                true
            };
            if let Some(mov) = best_move(&board, is_maximising, 5) {
                println!("best move is {mov}");
            } else {
                println!("No moves available");
            }
        }
        Err(err) => {
            println!("error in fen : {err}");
        }
    }
}
//