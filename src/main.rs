mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board, ChessMove};
use crate::search::alpha_beta::best_move;
use std::{io, time::Instant, time::Duration};
fn main() {
    println!("Enter fen string ");
    let max_time = Duration::new(1,0);//seconds and nano-seconds adjustments
    let mut fen:String = String::new();
    io::stdin().read_line(&mut fen).expect("Data not a string");

    let board_fen:Vec<&str> = fen.split_whitespace().collect();
    let piece_to_move = board_fen[1];//takes just the current player

        match Board::from_str(fen.as_str()){
        Ok(board) => {//checks condition to see if board is legal
            //checks whose turn it is currently and feeds to alpha beta
            let is_maximising = if piece_to_move == "b"{
                false
            }else{
                true
            };
            // iterative deepening code.
            let (mut best_mov,mut eval):(ChessMove, i32) = (ChessMove::default(), 0);

            let now = Instant::now();//starts the time.
            for depth_iterate in 1..100{//the timing logic can be fine-tuned a lot based on available time, position etc.
                let mut elapsed = now.elapsed();//checks if time constraint is passed.
                //the timing logic has to be changed to make sure live timing is possible so it takes only the specified amount of time.
                if elapsed<=max_time{
                    if let Some((mov, evaluation)) = best_move(&board, is_maximising, depth_iterate) {
                        (best_mov, eval) = (mov, evaluation);
                    } else {
                        println!("No moves available");
                        break;
                    }
                }else{
                    break
                }
            }
            println!("best move is {best_mov} with eval as {eval}");

            // let zobrist_table = ZobristHashing::new_table();
            // let current_hash = compute_hash_value(&board, &zobrist_table);
            // for mv in MoveGen::new_legal(&board){
            //     let hash = updated_hash_move(current_hash, &mv, &zobrist_table, &board);
            //     println!("{hash}");
            // }
            // let search = q_search(&board, i32::MIN, i32::MAX, 0, 3,true);
            // println!("{search}");
            // let eval = pe_sto(&board);
            // println!("{eval}")

        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
}
