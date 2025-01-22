mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board};
use crate::search::alpha_beta::best_move;
use crate::search::search_improvements::quiescent_search::tactical_moves;
fn main() {
    let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2"; //feed the fen to this...
    //checks whose turn it is currently and feeds to alpha beta
    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];//takes just the current player

        match Board::from_str(fen) {
        Ok(board) => {//checks condition to see current player if board is legal
            // let is_maximising = if piece_to_move == "b"{
            //     false
            // }else{
            //     true
            // };
            //
            //
            // if let Some((mov, eval)) = best_move(&board, is_maximising, 7) {
            //     println!("best move is {mov} with eval as {eval}");
            // } else {
            //     println!("No moves available");
            // }
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
