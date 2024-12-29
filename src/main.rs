mod evaluation;

use std::str::FromStr;
use chess::Board;
use crate::evaluation::alpha_beta::best_move;


fn main(){
    let fen = "3Q4/5ppk/2q1p2p/2b1PP2/2p1R1P1/r1N5/7P/2R4K b - - 0 1";
    match Board::from_str(fen){
        Ok(board) => {
            if let Some(mov) = best_move(&board, false, 6){
                println!("best move is {mov}");
            }else{
                println!("No moves available");
            }
            // println!("comment me out after testing");
            // let moves = moves_sorted(&board);
            // for pos_move in moves{
            //     println!("{pos_move}");
            // }
        }
        Err(err) => {
            println!("error in fen : {err}");
        }
    }
}
