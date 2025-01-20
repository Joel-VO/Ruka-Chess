mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board};
use crate::search::alpha_beta::best_move;
use crate::search::search_improvements::zobrist_hash::{ZobristHashing, compute_hash_value};
fn main() {
    let fen = "4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24"; //feed the fen to this...
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

            if let Some(mov) = best_move(&board, is_maximising, 6, hash, &zobrist_table) {
                println!("best move is {mov}");
            } else {
                println!("No moves available");
            }
            // let zobrist_table = ZobristHashing::new_table();
            // let current_hash = compute_hash_value(&board, &zobrist_table);
            // for mv in MoveGen::new_legal(&board){
            //     let hash = updated_hash_move(current_hash, &mv, &zobrist_table, &board);
            //     println!("{hash}");
            // }
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
}
