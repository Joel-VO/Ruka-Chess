mod evaluation;
mod search;

use std::str::FromStr;
use chess::Board;
use crate::search::alpha_beta::best_move;
use crate::search::search_improvements::zobrist_hash::{ZobristHashing, compute_hash_value};
fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"; //feed the fen to this...
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

            let zobrist_table = ZobristHashing::new_table();
            let hash = compute_hash_value(&board, &zobrist_table);

            if let Some(mov) = best_move(&board, is_maximising, 5, hash, &zobrist_table) {
                println!("best move is {mov}");
            } else {
                println!("No moves available");
            }
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
    let zobrist_table = ZobristHashing::new_table();
    println!("{0}",zobrist_table.piece_square[0][0])
}
