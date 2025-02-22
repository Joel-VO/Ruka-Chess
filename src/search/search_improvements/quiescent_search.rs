//add in quiescent search to negate the effects of the horizon effect
//will increase search times, but not much and moves will be better as critical captures are looked further into, improving the search result.
//not all nodes are looked into so not too expensive

use chess::{Board, BoardStatus, ChessMove, Color, MoveGen,Piece::Pawn};
use crate::evaluation::evaluations::{pe_sto, evaluation_func};

fn tactical_moves(board: &Board)->Vec<ChessMove>{
    //returns a Vec<ChessMove> of all legal captures in a position. Can be modified to add checks as well to improve search quality.
    let move_gen = MoveGen::new_legal(board);
    let mut avail_moves:Vec<ChessMove> = Vec::new();
    if board.checkers().popcnt()>0{
        avail_moves = move_gen.collect();
    }else{
        for moves in move_gen{
            let capture = board.piece_on(moves.get_dest());

            let en_passant:bool = if board.en_passant().is_some(){
                let dissimilar_file:bool = (moves.get_source().get_file()) != (moves.get_dest().get_file());
                let target_capture:bool = board.piece_on(moves.get_dest()) == None;
                let piece_start:bool = board.piece_on(moves.get_source()) == Some(Pawn);
                piece_start && dissimilar_file && target_capture
            }else{
                false
            };
            let promotion = moves.get_promotion();
            let check_moves:bool = if board.make_move_new(moves).checkers().popcnt()>0{
                true
            }else{
              false
            };

            if capture.is_some() || en_passant || promotion.is_some() || check_moves{
                avail_moves.push(moves);
            }
        }
    }
    avail_moves
}

pub fn q_search(board: &Board, mut alpha: i32, mut beta: i32, depth: u8, max_depth: u8, is_maximising: bool) -> i32 {
    // Terminal node checks
    if board.status() == BoardStatus::Checkmate {
        return if board.side_to_move() == Color::White {
            -400000 + (depth as i32)
        } else {
            400000 - (depth as i32)
        };
    } else if board.status() == BoardStatus::Stalemate {
        return 0;
    }

    // Stand-pat evaluation
    let stand_pat = evaluation_func(board);

    // Base case: max depth reached or no tactical moves
    let moves_tactical = tactical_moves(board);
    if depth >= max_depth || moves_tactical.is_empty() {
        return stand_pat;
    }

    // Initialize best value with stand-pat
    let mut best_val = stand_pat;

    if is_maximising {
        // Maximizing player logic
        if stand_pat >= beta {
            return stand_pat; // Beta cutoff
        }
        alpha = alpha.max(stand_pat);

        for mv in moves_tactical {
            let new_board = board.make_move_new(mv);
            let eval = q_search(&new_board, alpha, beta, depth + 1, max_depth, false);

            best_val = best_val.max(eval);
            alpha = alpha.max(best_val);

            if beta <= alpha {
                break; // Alpha cutoff
            }
        }
    } else {
        // Minimizing player logic
        if stand_pat <= alpha {
            return stand_pat; // Alpha cutoff
        }
        beta = beta.min(stand_pat);

        for mv in moves_tactical {
            let new_board = board.make_move_new(mv);
            let eval = q_search(&new_board, alpha, beta, depth + 1, max_depth, true);

            best_val = best_val.min(eval);
            beta = beta.min(best_val);

            if beta <= alpha {
                break; // Beta cutoff
            }
        }
    }

    best_val
}