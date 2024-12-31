mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;
use crate::evaluation::evaluations::evaluate;


fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1";
    match Board::from_str(fen) {
        Ok(board) => {
            // if let Some(mov) = best_move(&board, true, 2) {
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