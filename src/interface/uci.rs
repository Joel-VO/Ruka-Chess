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
            // Additional options can be listed here if needed
            println!("uciok");
        }else if  input == "ucinewgame"{
            TRANSPOSITION_TABLE.clear();
        }else if input == "isready" {// Ready check
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
                            }
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

            let tokens: Vec<&str> = input.split_whitespace().collect();
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
            let time_limit = if let Some(mt) = movetime {
                mt
            } else {
                let player_time = if is_maximising { wtime } else { btime };
                player_time.map_or(Duration::from_secs(2) / 30, |pt| {
                    if pt >= Duration::from_secs(600) {  // 10 minutes
                        Duration::from_secs(5)
                    } else if pt <= Duration::from_secs(60) {  // 1 minute
                        Duration::from_secs(1)
                    } else if pt <= Duration::from_secs(300) {  // 5 minutes
                        Duration::from_secs(2)
                    } else {
                        // For times between 5-10 minutes, use original division by 30 logic
                        pt / 30
                    }
                })
            };

            // Iterative deepening search
            let now = Instant::now();
            let mut best_mov = ChessMove::default();
            let mut _eval = 0;

            for depth in (2..100).step_by(2) {
                if now.elapsed() > time_limit {
                    break;
                }
                if let Some((mv, evaluation)) = best_move(&board, is_maximising, depth) {
                    best_mov = mv;
                    _eval = evaluation;
                } else {
                    break;
                }
            }

            println!("bestmove {}", best_mov);
        }
    }
}