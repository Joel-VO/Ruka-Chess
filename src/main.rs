mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;
use crate::evaluation::evaluations::evaluate;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";//mate in 11 for white
    //checks whose turn it is currently and feeds to alpha beta
    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];
        match Board::from_str(fen) {
        Ok(board) => {
            // let is_maximising = if piece_to_move == "b"{
            //     false
            // }else{
            //     true
            // };
            // if let Some(mov) = best_move(&board, is_maximising, 5) {
            //     println!("best move is {mov}");
            // } else {
            //     println!("No moves available");
            // }
            let eval = evaluate(&board);
            println!("{eval}");
        }
        Err(err) => {
            println!("error in fen : {err}");
        }
    }
}
