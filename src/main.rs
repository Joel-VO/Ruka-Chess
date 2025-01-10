mod evaluation;
mod search;

use std::str::FromStr;
use chess::Board;
// use crate::search::alpha_beta::best_move;
use crate::evaluation::evaluations::pe_sto;

fn main() {
    let fen = "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";//feed the fen to this...
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
            // if let Some(mov) = best_move(&board, is_maximising, 4) {
            //     println!("best move is {mov}");
            // } else {
            //     println!("No moves available");
            // }
            let eval = pe_sto(&board);
            println!("{eval}");
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
}
//found the bug is most likely due to the king not having an evaluation, so no means to identify king position.