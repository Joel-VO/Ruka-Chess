use chess::{Board, BoardStatus, ChessMove, Color};
use crate::search::move_ordering::moves_sorted;
use rayon::prelude::*;
use crate::search::search_improvements::quiescent_search::q_search;
use crate::search::search_improvements::lmr::lmr;
use crate::search::search_improvements::zobrist_hash::{compute_hash_value, updated_hash_move, Z_HASHING_KEYS, TRANSPOSITION_TABLE, NodeType, TtStructure};

pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8) ->Option<(ChessMove, i32)>{

    let alpha = i32::MIN;
    let beta = i32::MAX;

    //main_board init of hash
    let main_board_hash = compute_hash_value(board,&Z_HASHING_KEYS);


    //collecting possible root node moves(moves present in current position) and creates individual threads
    let legal_moves = moves_sorted(board);
    let (best_move, _best_eval) = legal_moves//(best_move, eval) returns a tuple with best_move and the eval
        .par_iter()//iterates and creates a thread
        .map(//searches using alpha beta and returns the value for each root node move thread
             |&moves|{
                 let current_position = board.make_move_new(moves);

                 // updation of the hash for each move.
                 let updated_board_hash = updated_hash_move(main_board_hash,&moves,&Z_HASHING_KEYS,board);


                 let eval = alpha_beta_search(&current_position, alpha, beta, !is_maximising, 1,max_depth,updated_board_hash);//added in updated_board_hash
                 (Some(moves),eval)//if move exists, then returns it
             }
        )
        .reduce(//this is the condition that's checked to combine different threads
                || {//initial case
                    if is_maximising{
                        (None, i32::MIN)
                    }else{
                        (None, i32::MAX)
                    }
                },
                |a,b|{
                    if is_maximising{//the condition to see what's the best move
                        if a.1>b.1{//a.1 is the eval
                            a
                        }else{
                            b
                        }
                    }else{
                        if a.1>b.1{
                            b
                        }else{
                            a
                        }
                    }
                }
        );
    if let Some(mov) = best_move {
        Some((mov, _best_eval))
    } else {
        None
    }
}

fn alpha_beta_search(board: &Board,
                     mut alpha: i32,
                     mut beta: i32,
                     is_maximising: bool,
                     depth: u8,
                     max_depth: u8,
                     current_hash: u64) -> i32 {

    // add in condition to check transposition table for hash computed in parent
    if let Some(entry) = TRANSPOSITION_TABLE.get(&current_hash) {
        if entry.depth >= max_depth-depth {
            match entry.node_type {
                NodeType::Exact => {return entry.score},
                NodeType::LowerBound => {if entry.score >= beta {return entry.score}}
                NodeType::UpperBound => {if entry.score <= alpha {return entry.score}}
            }
        }
    }
    if board.status() == BoardStatus::Checkmate {
        if board.side_to_move() == Color::White {
            -500000 + (depth as i32)
        } else {
            500000 - (depth as i32)
        }
    }else if board.status() == BoardStatus::Stalemate{
        return 0
    }else if depth >= max_depth{
        return q_search(board, alpha, beta, depth, max_depth+2, is_maximising)//made max_depth even due to odd even rule
    }else{
        let original_alpha = alpha;
        let original_beta = beta;
        let eval: i32;

        if is_maximising{
            let mut max_eval = i32::MIN;
            let mut first_move:bool = true;
            let legal_moves = moves_sorted(board);//pv node is used for LMP, delete this to go back to normal search mode

            //new added LMR
            let lmr_depth:u8 = lmr(&board,&legal_moves, depth)+depth;
            // after the first few moves only add in the lmr concept. don't want reduced depth for better moves.

            let mut move_number = 0;

            for mv in legal_moves {
                let updated_board_hash = updated_hash_move(current_hash, &mv, &Z_HASHING_KEYS, board);
                move_number += 1;
                let current_position = board.make_move_new(mv);

                let current_eval = if first_move {
                    first_move = false;
                    alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth, updated_board_hash)
                } else {
                    // Regular PVS with LMR
                    let search_depth = if move_number < 4 { depth+1 } else { lmr_depth+1 };
                    let pvs_eval = alpha_beta_search(&current_position, alpha, alpha+1, false, search_depth, max_depth, updated_board_hash);

                    if pvs_eval >= beta{
                        pvs_eval
                    }else if pvs_eval > alpha{
                        alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth, updated_board_hash)//full search, no LMR
                    }else{
                        pvs_eval
                    }
                };

                max_eval = max_eval.max(current_eval);
                alpha = alpha.max(current_eval);

                if beta <= alpha {
                    break;
                }
            }
            eval = max_eval


        }else {
            let mut min_eval = i32::MAX;
            let legal_moves = moves_sorted(board);
            let lmr_depth = lmr(&board, &legal_moves, depth) + depth;
            let mut first_move = true;
            let mut move_number = 0;

            for mv in legal_moves {
                let updated_board_hash = updated_hash_move(current_hash, &mv, &Z_HASHING_KEYS, board);
                move_number += 1;
                let current_position = board.make_move_new(mv);

                let current_eval = if first_move {
                    first_move = false;
                    alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth, updated_board_hash)
                } else {
                    // Regular PVS with LMR
                    let search_depth = if move_number < 4 { depth+1 } else { lmr_depth+1 };
                    let pvs_eval = alpha_beta_search(&current_position, beta-1, beta, true, search_depth, max_depth, updated_board_hash);

                    if pvs_eval <= alpha{
                        pvs_eval
                    }else if pvs_eval < beta{
                        alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth, updated_board_hash)//full search with LMR removed
                    }else{
                        pvs_eval
                    }
                };

                min_eval = min_eval.min(current_eval);
                beta = beta.min(current_eval);

                if beta <= alpha {
                    break;
                }
            }

            eval = min_eval;
        }
        let flag = if is_maximising{
            if eval <= original_alpha{
                NodeType::UpperBound
            }else if eval >= beta{
                NodeType::LowerBound
            }else{
                NodeType::Exact
            }
        }else{
            if eval >= original_beta{
                NodeType::UpperBound
            }else if eval <= alpha{
                NodeType::LowerBound
            }else{
                NodeType::Exact
            }
        };
        TRANSPOSITION_TABLE.insert(current_hash,TtStructure{
            score:eval,
            depth:max_depth-depth,
            node_type:flag
        });
        eval
    }
}

// https://chess.stackexchange.com/questions/42612/chess-programming-have-to-clear-my-transposition-table-after-every-move#comment70276_42612