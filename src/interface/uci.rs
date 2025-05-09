// use std::str::FromStr;
// use chess::{Board, ChessMove, Color};
// use crate::search::alpha_beta::best_move;
// use std::{io, time::{Duration, Instant}};
// use crate::search::search_improvements::zobrist_hash::{TRANSPOSITION_TABLE, Z_HASHING_KEYS};
//
// pub fn uci() {
//     // Default board is the starting position
//     let mut board = Board::default();
//     let stdin = io::stdin();
//     let _ = *TRANSPOSITION_TABLE; //init of Transposition table
//     let _ = *Z_HASHING_KEYS; // init of hashing keys
//
//     // UCI main loop
//     loop {
//         let mut input = String::new();
//         if stdin.read_line(&mut input).is_err() {
//             break;
//         }
//         let input = input.trim();
//         if input.is_empty() {
//             continue;
//         }
//
//         // Quit command
//         if input == "quit" {
//             TRANSPOSITION_TABLE.clear();
//             break;
//         }
//         // UCI initialization
//         else if input == "uci" {
//             println!("id name Ruka-Chess");
//             println!("id author Joel-VO");
//             // Additional options can be listed here if needed
//             println!("uciok");
//         }else if  input == "ucinewgame"{
//             TRANSPOSITION_TABLE.clear();
//         }else if input == "isready" {// Ready check
//             println!("readyok");
//         }
//         // Position command
//         else if input.starts_with("position") {
//             // Split the input into tokens
//             let tokens: Vec<&str> = input.split_whitespace().collect();
//             if tokens.len() < 2 {
//                 continue;
//             }
//             if tokens[1] == "startpos" {
//                 board = Board::default();
//                 // Check for subsequent moves
//                 if let Some(moves_index) = tokens.iter().position(|&s| s == "moves") {
//                     for mv_str in &tokens[(moves_index + 1)..] {
//                         match ChessMove::from_str(mv_str) {
//                             Ok(chess_move) => {
//                                 // Apply the move if legal (assuming make_move_new returns the updated board)
//                                 board = board.make_move_new(chess_move);
//                             }
//                             Err(err) => {
//                                 eprintln!("Invalid move {mv_str}: {err}");
//                             }
//                         }
//                     }
//                 }
//             } else if tokens[1] == "fen" {
//                 // Collect all tokens after "fen" until the optional "moves" keyword
//                 let mut fen_parts = Vec::new();
//                 let mut i = 2;
//                 while i < tokens.len() && tokens[i] != "moves" {
//                     fen_parts.push(tokens[i]);
//                     i += 1;
//                 }
//                 let fen_string = fen_parts.join(" ");
//                 match Board::from_str(&fen_string) {
//                     Ok(b) => board = b,
//                     Err(e) => {
//                         eprintln!("Invalid FEN: {e}");
//                         continue;
//                     }
//                 }
//                 // Apply moves if any were provided
//                 if i < tokens.len() && tokens[i] == "moves" {
//                     for mv_str in &tokens[(i + 1)..] {
//                         match ChessMove::from_str(mv_str) {
//                             Ok(chess_move) => {
//                                 board = board.make_move_new(chess_move);
//                             }
//                             Err(err) => {
//                                 eprintln!("Invalid move {mv_str}: {err}");
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//         // Go command: start the search
//         else if input.starts_with("go") {
//             // Default move time is 1 second.
//             let mut movetime: Option<Duration> = None;
//             let mut wtime: Option<Duration> = None;
//             let mut btime: Option<Duration> = None;
//
//             let tokens: Vec<&str> = input.split_whitespace().collect();
//             // Parse time parameters
//             if let Some(index) = tokens.iter().position(|&s| s == "movetime") {
//                 if index + 1 < tokens.len() {
//                     if let Ok(ms) = tokens[index + 1].parse::<u64>() {
//                         movetime = Some(Duration::from_millis(ms));
//                     }
//                 }
//             }
//             if let Some(index) = tokens.iter().position(|&s| s == "wtime") {
//                 if index + 1 < tokens.len() {
//                     if let Ok(ms) = tokens[index + 1].parse::<u64>() {
//                         wtime = Some(Duration::from_millis(ms));
//                     }
//                 }
//             }
//             if let Some(index) = tokens.iter().position(|&s| s == "btime") {
//                 if index + 1 < tokens.len() {
//                     if let Ok(ms) = tokens[index + 1].parse::<u64>() {
//                         btime = Some(Duration::from_millis(ms));
//                     }
//                 }
//             }
//
//             // Determine whose turn it is
//             let is_maximising = matches!(board.side_to_move(), Color::White);
//
//             // Calculate time limit based on current player's remaining time
//             let time_limit = if let Some(mt) = movetime {
//                 mt
//             } else {
//                 let player_time = if is_maximising { wtime } else { btime };
//                 player_time.map_or(Duration::from_secs(2) / 30, |pt| {
//                     if pt >= Duration::from_secs(600) {  // 10 minutes
//                         Duration::from_secs(5)
//                     } else if pt <= Duration::from_secs(60) {  // 1 minute
//                         Duration::from_secs(1)
//                     } else if pt <= Duration::from_secs(300) {  // 5 minutes
//                         Duration::from_secs(2)
//                     } else {
//                         // For times between 5-10 minutes, use original division by 30 logic
//                         pt / 30
//                     }
//                 })
//             };
//
//             // Iterative deepening search
//             let now = Instant::now();
//             let mut best_mov = ChessMove::default();
//             let mut _eval = 0;
//
//             for depth in (1..100).step_by(2) {
//                 if now.elapsed() > time_limit {
//                     break;
//                 }
//                 if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
//                     best_mov = mv;
//                     _eval = evaluation;
//                 } else {
//                     break;
//                 }
//             }
//
//             println!("bestmove {}", best_mov);
//
//             // let now = Instant::now();
//             // let mut best_mov = ChessMove::default();
//             // let mut _eval = 0;
//             // let mut prev_depth_time = Duration::from_millis(0);
//             //
//             // // Start with depth 1 instead of 2 to have a valid move faster
//             // for depth in 1..100 {
//             //     let depth_start = Instant::now();
//             //
//             //     // Check if we have enough time for the next iteration
//             //     // If previous depth took more than 1/3 of our remaining time, stop
//             //     if depth > 1 && prev_depth_time > (time_limit - now.elapsed()) / 3 {
//             //         break;
//             //     }
//             //
//             //     // Break if we've used 80% of our allocated time
//             //     if now.elapsed() > time_limit.mul_f32(0.8) {
//             //         break;
//             //     }
//             //
//             //     if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
//             //         best_mov = mv;
//             //         _eval = evaluation;
//             //
//             //         // Print info about current search depth
//             //         // println!("info depth {} score cp {} time {} pv {}",
//             //         //          depth,
//             //         //          evaluation,
//             //         //          now.elapsed().as_millis(),
//             //         //          best_mov);
//             //     } else {
//             //         break;
//             //     }
//             //
//             //     prev_depth_time = depth_start.elapsed();
//             // }
//             //
//             // println!("bestmove {}", best_mov);
//         }
//     }
// }
use std::str::FromStr;
use chess::{Board, ChessMove, Color};
use crate::search::alpha_beta::best_move;
use std::{io, time::{Duration, Instant}};
use crate::search::search_improvements::zobrist_hash::{TRANSPOSITION_TABLE, Z_HASHING_KEYS};

pub fn uci() {
    // Default board is the starting position
    let mut board = Board::default();
    let stdin = io::stdin();
    let _ = *TRANSPOSITION_TABLE; //init of Transposition table
    let _ = *Z_HASHING_KEYS; // init of hashing keys

    // UCI main loop
    loop {
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Quit command
        if input == "quit" {
            TRANSPOSITION_TABLE.clear();
            break;
        }
        // UCI initialization
        else if input == "uci" {
            println!("id name Ruka-Chess");
            println!("id author Joel-VO");
            println!("option name Threads type spin default 1 min 1 max 128");
            println!("option name Hash type spin default 16 min 1 max 32768");
            println!("option name MultiPV type spin default 1 min 1 max 500");
            println!("option name UCI_Chess960 type check default false");
            println!("option name Move Overhead type spin default 30 min 0 max 5000");
            println!("option name Slow Mover type spin default 84 min 10 max 1000");
            println!("uciok");
        }
        else if input == "ucinewgame" {
            TRANSPOSITION_TABLE.clear();
        }
        else if input == "isready" {
            // Ready check
            println!("readyok");
        }
        // Position command
        else if input.starts_with("position") {
            // Split the input into tokens
            let tokens: Vec<&str> = input.split_whitespace().collect();
            if tokens.len() < 2 {
                continue;
            }

            if tokens[1] == "startpos" {
                board = Board::default();
                // Check for subsequent moves
                if let Some(moves_index) = tokens.iter().position(|&s| s == "moves") {
                    for mv_str in &tokens[(moves_index + 1)..] {
                        match ChessMove::from_str(mv_str) {
                            Ok(chess_move) => {
                                // Apply the move if legal
                                board = board.make_move_new(chess_move);
                            },
                            Err(err) => {
                                eprintln!("Invalid move {mv_str}: {err}");
                            }
                        }
                    }
                }
            } else if tokens[1] == "fen" {
                // Collect all tokens after "fen" until the optional "moves" keyword
                let mut fen_parts = Vec::new();
                let mut i = 2;
                while i < tokens.len() && tokens[i] != "moves" {
                    fen_parts.push(tokens[i]);
                    i += 1;
                }

                let fen_string = fen_parts.join(" ");
                match Board::from_str(&fen_string) {
                    Ok(b) => board = b,
                    Err(e) => {
                        eprintln!("Invalid FEN: {e}");
                        continue;
                    }
                }

                // Apply moves if any were provided
                if i < tokens.len() && tokens[i] == "moves" {
                    for mv_str in &tokens[(i + 1)..] {
                        match ChessMove::from_str(mv_str) {
                            Ok(chess_move) => {
                                board = board.make_move_new(chess_move);
                            },
                            Err(err) => {
                                eprintln!("Invalid move {mv_str}: {err}");
                            }
                        }
                    }
                }
            }
        }
        // Go command: start the search
        else if input.starts_with("go") {
            // Default move time is 1 second.
            let mut movetime: Option<Duration> = None;
            let mut wtime: Option<Duration> = None;
            let mut btime: Option<Duration> = None;
            let mut infinite = false;

            let tokens: Vec<&str> = input.split_whitespace().collect();

            // Check for infinite search
            if tokens.contains(&"infinite") {
                infinite = true;
            }

            // Parse time parameters
            if let Some(index) = tokens.iter().position(|&s| s == "movetime") {
                if index + 1 < tokens.len() {
                    if let Ok(ms) = tokens[index + 1].parse::<u64>() {
                        movetime = Some(Duration::from_millis(ms));
                    }
                }
            }

            if let Some(index) = tokens.iter().position(|&s| s == "wtime") {
                if index + 1 < tokens.len() {
                    if let Ok(ms) = tokens[index + 1].parse::<u64>() {
                        wtime = Some(Duration::from_millis(ms));
                    }
                }
            }

            if let Some(index) = tokens.iter().position(|&s| s == "btime") {
                if index + 1 < tokens.len() {
                    if let Ok(ms) = tokens[index + 1].parse::<u64>() {
                        btime = Some(Duration::from_millis(ms));
                    }
                }
            }

            // Determine whose turn it is
            let is_maximising = matches!(board.side_to_move(), Color::White);

            // Calculate time limit based on current player's remaining time
            let time_limit = if infinite {
                Duration::from_secs(3600) // Use a very long time for infinite search
            } else if let Some(mt) = movetime {
                mt
            } else {
                let player_time = if is_maximising { wtime } else { btime };
                player_time.map_or(Duration::from_secs(2), |pt| {
                    // Estimate remaining moves based on game phase
                    // Count pieces to roughly determine game phase
                    let piece_count = board.combined().popcnt() as u32;

                    // Early game: ~40 moves remaining, middlegame: ~25, endgame: ~15
                    let estimated_moves_left = if piece_count > 24 {
                        40 // Early game
                    } else if piece_count > 10 {
                        25 // Middle game
                    } else {
                        15 // End game
                    };

                    // Base time allocation: time_remaining / estimated_moves_left
                    let base_time = pt / estimated_moves_left;

                    // Add safety margin - never use more than 20% of remaining time
                    let max_time = pt / 5;

                    // Set minimum thinking time based on remaining time
                    let min_time = if pt >= Duration::from_secs(300) { // >5 minutes
                        Duration::from_millis(500)
                    } else if pt >= Duration::from_secs(60) { // >1 minute
                        Duration::from_millis(200)
                    } else {
                        Duration::from_millis(100) // Critical time
                    };

                    // Clamp the time between min and max values
                    if base_time < min_time {
                        min_time
                    } else if base_time > max_time {
                        max_time
                    } else {
                        base_time
                    }
                })
            };

            // Iterative deepening search with improved time management
            let now = Instant::now();
            let mut best_mov = ChessMove::default();
            let mut _eval = 0;
            let mut prev_depth_time = Duration::from_millis(0);
            let mut prev_best_move = None;
            let mut stability_counter = 0;

            // Start with depth 1 instead of 2 to have a valid move faster
            for depth in 1..100 {
                let depth_start = Instant::now();

                // Check if we have enough time for the next iteration
                // If previous depth took more than 1/3 of our remaining time, stop
                if depth > 1 && prev_depth_time > (time_limit - now.elapsed()) / 3 && !infinite {
                    break;
                }

                // Break if we've used 80% of our allocated time
                if now.elapsed() > time_limit.mul_f32(0.8) && !infinite {
                    break;
                }

                if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
                    // Check move stability - if same move is found multiple times, we can exit earlier
                    if let Some(prev_mv) = prev_best_move {
                        if prev_mv == mv {
                            stability_counter += 1;

                            // If we have high stability (same move for 3+ iterations) and used >50% time, we can exit
                            if stability_counter >= 3 && now.elapsed() > time_limit.mul_f32(0.5) && !infinite {
                                best_mov = mv;
                                _eval = evaluation;
                                break;
                            }
                        } else {
                            stability_counter = 0;
                        }
                    }

                    best_mov = mv;
                    _eval = evaluation;
                    prev_best_move = Some(mv);

                    // Print info about current search depth
                    println!("info depth {} score cp {} time {} pv {}",
                             depth,
                             evaluation,
                             now.elapsed().as_millis(),
                             best_mov);
                } else {
                    break;
                }

                prev_depth_time = depth_start.elapsed();

                // For infinite search, we don't break based on time
                if !infinite && now.elapsed() >= time_limit {
                    break;
                }
            }

            println!("bestmove {}", best_mov);
        }
    }
}
