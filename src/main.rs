mod evaluation;
mod search;

use std::str::FromStr;
use chess::{Board, ChessMove, Color};
use crate::search::alpha_beta::best_move;
use std::{io, time::{Duration, Instant}};

fn main() {
    // Default board is the starting position
    let mut board = Board::default();
    let stdin = io::stdin();

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
            break;
        }
        // UCI initialization
        else if input == "uci" {
            println!("id name Ruka-Chess");
            println!("id author Joel-VO");
            // Additional options can be listed here if needed
            println!("uciok");
        }
        // Ready check
        else if input == "isready" {
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
                                // Apply the move if legal (assuming make_move_new returns the updated board)
                                board = board.make_move_new(chess_move);
                            }
                            Err(err) => {
                                eprintln!("Invalid move {}: {}", mv_str, err);
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
                        eprintln!("Invalid FEN: {}", e);
                        continue;
                    }
                }
                // Apply moves if any were provided
                if i < tokens.len() && tokens[i] == "moves" {
                    for mv_str in &tokens[(i + 1)..] {
                        match ChessMove::from_str(mv_str) {
                            Ok(chess_move) => {
                                board = board.make_move_new(chess_move);
                            }
                            Err(err) => {
                                eprintln!("Invalid move {}: {}", mv_str, err);
                            }
                        }
                    }
                }
            }
        }
        // Go command: start the search
        else if input.starts_with("go") {
            // Default move time is 1 second.
            let movetime = Duration::new(5, 0);//changed from mut to immutable. change back after work is done.
            // let tokens: Vec<&str> = input.split_whitespace().collect();
            // // If a "movetime" parameter is provided, use it (expecting time in milliseconds)
            // if let Some(index) = tokens.iter().position(|&s| s == "movetime") {
            //     if index + 1 < tokens.len() {
            //         if let Ok(ms) = tokens[index + 1].parse::<u64>() {
            //             movetime = Duration::from_millis(ms);
            //         }
            //     }
            // }

            // Determine whose turn it is (assume white is maximising)
            let is_maximising = match board.side_to_move() {
                Color::White => true,
                Color::Black => false,
            };

            // Iterative deepening search
            let now = Instant::now();
            let mut best_mov = ChessMove::default();
            let mut eval = 0;
            // You can adjust the depth range as needed.
            for depth in 7..100 {
                if now.elapsed() > movetime {
                    // Optionally, print depth info for debugging:
                    // eprintln!("Reached depth {} after {:?}", depth, now.elapsed());
                    break;
                }
                if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
                    best_mov = mv;
                    eval = evaluation;
                } else {
                    break;
                }
            }
            // Output the best move in UCI format.
            println!("bestmove {}", best_mov);
        }
    }
}