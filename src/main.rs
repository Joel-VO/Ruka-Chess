mod evaluation;
mod search;
mod interface;
use interface::uci::uci;
use std::env;


// use crate::search::search_improvements::zobrist_hash::{TRANSPOSITION_TABLE, Z_HASHING_KEYS};
// use crate::search::alpha_beta::best_move;
// use std::str::FromStr;
// use chess::{Board, ChessMove};
// use std::{io, time::{Duration, Instant}};
// // use search::search_improvements::zobrist_hash::{TRANSPOSITION_TABLE};

fn main() {

    env::set_var("RAYON_NUM_THREADS", "24");
    uci();
    
    // let _ = *TRANSPOSITION_TABLE; //init of Transposition table
    // let _ = *Z_HASHING_KEYS; // init of hashing keys
    // 
    // TRANSPOSITION_TABLE.clear();
    // println!("Enter fen string ");
    // let max_time = Duration::new(5,0);//seconds and nanoseconds adjustments
    // let mut fen:String = String::new();
    // io::stdin().read_line(&mut fen).expect("Data not a string");
    // //  4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24
    // // rn3r1k/p2q4/bp2pp1p/3pP3/P2NRQ2/1Pb2NPP/5PB1/3R2K1 w - - 1 22
    // // rnbqkbnr/pp1pppp1/8/2pP3p/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3
    // // R4r1k/6pp/2pq4/2n2b2/2Q1pP1b/1r2P2B/NP5P/2B2KNR b - - 1 24
    // // 8/8/8/8/8/8/P1k5/K7 w - - 0 1
    // let board_fen:Vec<&str> = fen.split_whitespace().collect();
    // let piece_to_move = board_fen[1];//takes just the current player
    // match Board::from_str(fen.as_str()){
    //     Ok(board) => {//checks condition to see if board is legal
    //         //checks whose turn it is currently and feeds to alpha beta
    //         let is_maximising = if piece_to_move == "b"{
    //             false
    //         }else{
    //             true
    //         };
    //         // iterative deepening code.
    //         let (mut best_mov,mut eval):(ChessMove, i32) = (ChessMove::default(), 0);
    // 
    //         let now = Instant::now();//starts the time.
    //         for depth_iterate in (6..100).step_by(2){//the timing logic can be fine-tuned a lot based on available time, position etc.
    //             println!("current depth: {depth_iterate}");
    //             let elapsed = now.elapsed();//checks if time constraint is passed.
    //             //the timing logic has to be changed to make sure live timing is possible so it takes only the specified amount of time.
    //             if elapsed<=max_time{
    //                 if let Some((mov, evaluation)) = best_move(&board, is_maximising, depth_iterate) {
    //                     (best_mov, eval) = (mov, evaluation);
    //                 } else {
    //                     println!("No moves available");
    //                     break;
    //                 }
    //             }else{
    //                 println!("max depth was: {depth_iterate}");
    //                 break
    //             }
    //         }
    //         println!("finished in {:.2?}", now.elapsed());
    //         println!("best move is {best_mov} with eval as {eval}");
    // 
    // 
    // //
    // //
    // //         // // rnb1kbnr/pppp1ppp/8/4p1N1/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 3
    // //         // //
    // //         // let mut new_test_board = Board::default();
    // //         // new_test_board = new_test_board.make_move_new(ChessMove::new(Square::E2,Square::E4, None));
    // //         // new_test_board = new_test_board.make_move_new(ChessMove::new(Square::E7,Square::E5, None));
    // //         // new_test_board = new_test_board.make_move_new(ChessMove::new(Square::G1,Square::F3, None));
    // //         // new_test_board = new_test_board.make_move_new(ChessMove::new(Square::D8,Square::G5, None));
    // //         // new_test_board = new_test_board.make_move_new(ChessMove::new(Square::F3,Square::G5, None));
    // //         //
    // //         // let zobrist_hashing_keys = ZobristHashing::new_table();
    // //         // println!("The values of the positions for each piece");
    // //         // let hash = compute_hash_value(&board, &zobrist_hashing_keys);
    // //         // let test_hash = compute_hash_value(&new_test_board, &zobrist_hashing_keys);
    // //         // println!("{hash} and {test_hash}");
    // //         //
    // 
    // 
    // 
    //     }
    //     Err(err) => {
    //         println!("error in fen : {err}");//can be changed later to make sure errors are handled
    //     }
    // }
}
//need to add testing 

//Make the thread count equal to the number of cpu threads present or slightly less(ideally)
//After running and going through the code, if any suggestion are present, make a pull request
//Leave some of the comments in the code, they're to explain future ideas
//Urls are to be kept there, they either explain whats to be done... or explain a concept


