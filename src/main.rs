mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";//feed the fen to this...
    //checks whose turn it is currently and feeds to alpha beta
    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];//takes just the current player

        match Board::from_str(fen) {
        Ok(board) => {//checks condition to see current player if board is legal
            let is_maximising = if piece_to_move == "b"{
                false
            }else{
                true
            };
            if let Some(mov) = best_move(&board, is_maximising, 7) {
                println!("best move is {mov}");
            } else {
                println!("No moves available");
            }
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
}
