mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board, ChessMove, Color};
use crate::search::alpha_beta::best_move;
use std::{io, time::{Duration, Instant}};
// use evaluation::evaluations::evaluation_func;

fn main() {
    //Default board is the starting position
    // let mut board = Board::default();
    // let stdin = io::stdin();
    //
    // // UCI main loop
    // loop {
    //     let mut input = String::new();
    //     if stdin.read_line(&mut input).is_err() {
    //         break;
    //     }
    //     let input = input.trim();
    //     if input.is_empty() {
    //         continue;
    //     }
    //
    //     // Quit command
    //     if input == "quit" {
    //         break;
    //     }
    //     // UCI initialization
    //     else if input == "uci" {
    //         println!("id name Ruka-Chess");
    //         println!("id author Joel-VO");
    //         // Additional options can be listed here if needed
    //         println!("uciok");
    //     }
    //     // Ready check
    //     else if input == "isready" {
    //         println!("readyok");
    //     }
    //     // Position command
    //     else if input.starts_with("position") {
    //         // Split the input into tokens
    //         let tokens: Vec<&str> = input.split_whitespace().collect();
    //         if tokens.len() < 2 {
    //             continue;
    //         }
    //         if tokens[1] == "startpos" {
    //             board = Board::default();
    //             // Check for subsequent moves
    //             if let Some(moves_index) = tokens.iter().position(|&s| s == "moves") {
    //                 for mv_str in &tokens[(moves_index + 1)..] {
    //                     match ChessMove::from_str(mv_str) {
    //                         Ok(chess_move) => {
    //                             // Apply the move if legal (assuming make_move_new returns the updated board)
    //                             board = board.make_move_new(chess_move);
    //                         }
    //                         Err(err) => {
    //                             eprintln!("Invalid move {mv_str}: {err}");
    //                         }
    //                     }
    //                 }
    //             }
    //         } else if tokens[1] == "fen" {
    //             // Collect all tokens after "fen" until the optional "moves" keyword
    //             let mut fen_parts = Vec::new();
    //             let mut i = 2;
    //             while i < tokens.len() && tokens[i] != "moves" {
    //                 fen_parts.push(tokens[i]);
    //                 i += 1;
    //             }
    //             let fen_string = fen_parts.join(" ");
    //             match Board::from_str(&fen_string) {
    //                 Ok(b) => board = b,
    //                 Err(e) => {
    //                     eprintln!("Invalid FEN: {e}");
    //                     continue;
    //                 }
    //             }
    //             // Apply moves if any were provided
    //             if i < tokens.len() && tokens[i] == "moves" {
    //                 for mv_str in &tokens[(i + 1)..] {
    //                     match ChessMove::from_str(mv_str) {
    //                         Ok(chess_move) => {
    //                             board = board.make_move_new(chess_move);
    //                         }
    //                         Err(err) => {
    //                             eprintln!("Invalid move {mv_str}: {err}");
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // Go command: start the search
    //     else if input.starts_with("go") {
    //         // Default move time is 1 second.
    //         let movetime = Duration::new(2, 0);//changed from mut to immutable. change back after work is done.
    //         // let tokens: Vec<&str> = input.split_whitespace().collect();
    //         // // If a "movetime" parameter is provided, use it (expecting time in milliseconds)
    //         // if let Some(index) = tokens.iter().position(|&s| s == "movetime") {
    //         //     if index + 1 < tokens.len() {
    //         //         if let Ok(ms) = tokens[index + 1].parse::<u64>() {
    //         //             movetime = Duration::from_millis(ms);
    //         //         }
    //         //     }
    //         // }
    //
    //         // Determine whose turn it is (assume white is maximising)
    //         let is_maximising = match board.side_to_move() {
    //             Color::White => true,
    //             Color::Black => false,
    //         };
    //
    //         // Iterative deepening search
    //         let now = Instant::now();
    //         let mut best_mov = ChessMove::default();
    //         let mut _eval = 0;
    //         // You can adjust the depth range as needed.
    //         for depth in (6..100).step_by(2){
    //             if now.elapsed() > movetime {
    //                 // Optionally, print depth info for debugging:
    //                 // eprintln!("Reached depth {depth} after {:?}",now.elapsed());
    //                 break;
    //             }
    //             if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
    //                 best_mov = mv;
    //                 _eval = evaluation;
    //             } else {
    //                 break;
    //             }
    //         }
    //         // Output the best move in UCI format.
    //         println!("bestmove {}", best_mov);
    //     }
    // }


    println!("Enter fen string ");
    let max_time = Duration::new(1,0);//seconds and nanoseconds adjustments
    let mut fen:String = String::new();
    io::stdin().read_line(&mut fen).expect("Data not a string");
    //  4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24
    // rn3r1k/p2q4/bp2pp1p/3pP3/P2NRQ2/1Pb2NPP/5PB1/3R2K1 w - - 1 22
    // rnbqkbnr/pp1pppp1/8/2pP3p/8/8/PPP1PPPP/RNBQKBNR w KQkq c6 0 3
    // R4r1k/6pp/2pq4/2n2b2/2Q1pP1b/1r2P2B/NP5P/2B2KNR b - - 1 24
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
            for depth_iterate in (6..100).step_by(2){//the timing logic can be fine-tuned a lot based on available time, position etc.
                let elapsed = now.elapsed();//checks if time constraint is passed.
                //the timing logic has to be changed to make sure live timing is possible so it takes only the specified amount of time.
                if elapsed<=max_time{
                    if let Some((mov, evaluation)) = best_move(&board, is_maximising, depth_iterate) {
                        (best_mov, eval) = (mov, evaluation);
                    } else {
                        println!("No moves available");
                        break;
                    }
                }else{
                    println!("max depth was: {depth_iterate}");
                    break
                }
            }
            println!("best move is {best_mov} with eval as {eval}");

            //don't delete yet...the eval extensions has to be updated further.
            // let eval = additional_eval_capability(&board, 0,24);
            // println!("{eval}");
        }
        Err(err) => {
            println!("error in fen : {err}");//can be changed later to make sure errors are handled
        }
    }
    //delete from this
}
//apply odd-even rule. makes sure search ends in the odd ply instead of the even one for stability.
//could add testing https://github.com/lithander/Leorik/blob/master/Leorik.Test/see.epd

//incorporate wtime and btime to timing and adjust time to search based on that.
//right now its very static, not at all dynamic based on position.
