use chess::{Board, BoardStatus, ChessMove, Color};
use crate::search::move_ordering::moves_sorted;
use rayon::prelude::*;//implements parallelization
use crate::search::search_improvements::quiescent_search::q_search;
use crate::search::search_improvements::lmr::lmr;

pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8)->Option<(ChessMove, i32)>{
    std::env::set_var("RAYON_NUM_THREADS", "16");
    let alpha = i32::MIN;
    let beta = i32::MAX;

    //collecting possible root node moves(moves present in current position) and creates individual threads
    let legal_moves:Vec<ChessMove> = moves_sorted(board);
    let (best_move, _best_eval) = legal_moves//(best_move, eval) returns a tuple with best_move and the eval
        .par_iter()//iterates and creates a thread
        .map(//searches using alpha beta and returns the value for each root node move thread
            |&moves|{
                let current_position = board.make_move_new(moves);
                let eval = alpha_beta_search(&current_position, alpha, beta, !is_maximising, 1,max_depth);//not sure of the true here
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

fn alpha_beta_search(board:&Board, mut alpha:i32, mut beta:i32, is_maximising:bool,
                     depth:u8, max_depth:u8) ->i32{//add in current_hash:u64

    //add in condition to check transposition table for hash computed in parent
    if board.status() == BoardStatus::Checkmate{ //checks checkmate condition first, then draw conditions
        if board.side_to_move() == Color::White{
            -400000 + (depth as i32)
        }else{
            400000 - (depth as i32)
        }
    }else if board.status() == BoardStatus::Stalemate{
        return 0
    }else if depth >= max_depth{
        return q_search(board, alpha, beta, depth, max_depth+3, is_maximising)
    }else{
        if is_maximising{
            let mut max_eval = i32::MIN;
            let mut first_move:bool = true;
            let legal_moves = moves_sorted(board);//pv node is used for LMP, delete this to go back to normal search mode

            //new added LMR
            let lmr_depth:u8 = {
                let added_val:u8 = lmr(&board,&legal_moves, depth);
                depth+added_val
            };

            for mv in legal_moves{

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth)//PV node so full search with no LMR
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, alpha, alpha+1, false, lmr_depth+1, max_depth);//PVS with LMR
                    if pvs_eval >= beta{
                        pvs_eval
                    }else if pvs_eval > alpha{
                        alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth)//full search, no LMR
                    }else{
                        pvs_eval
                    }
                };

                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);

                if beta<=alpha{
                    break;
                }
            }
            max_eval


        }else{
            let mut min_eval= i32::MAX;
            let legal_moves = moves_sorted(board);//added depth
            let mut first_move = true;

            //new added LMR
            let lmr_depth:u8 = {
                let added_val:u8 = lmr(&board,&legal_moves, depth);
                depth+added_val
            };

            for mv in legal_moves{

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth)//PVS with no LMR
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, beta-1, beta, true, lmr_depth+1, max_depth);//PVS with LMR
                    if pvs_eval <= alpha{
                        pvs_eval
                    }else if pvs_eval < beta{
                        alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth)//full search with LMR removed
                    }else{
                        pvs_eval
                    }
                };

                min_eval = min_eval.min(eval);
                beta = beta.min(eval);

                if beta<=alpha{
                    break;
                }
            }
            min_eval
        }
    }
}

fn abs(board:&Board, mut alpha:i32, mut beta:i32, is_maximising:bool,
                     depth:u8, max_depth:u8) ->i32{//add in current_hash:u64

    match board.status(){
        BoardStatus::Checkmate =>{
            if board.side_to_move() == Color::White{
                -400000 + (depth as i32)
            }else{
                400000 - (depth as i32)
            }
        }
        BoardStatus::Stalemate =>{
            return 0
        }
        _ => {
            if depth >= max_depth{
                q_search(board, alpha, beta, depth, max_depth+3, is_maximising)
            }else{
                //alpha-beta code
                let mut first_move:bool = true;//pvs first move
                let legal_moves = moves_sorted(board);
                let lmr_depth:u8 = lmr(&board,&legal_moves, depth)+depth;
                let best_eval = if is_maximising {i32::MIN} else {i32::MIN};
                for mv in legal_moves {
                    let current_position:Board = board.make_move_new(mv);
                    let eval = if first_move{
                        first_move=false;
                        let current_position:Board = board.make_move_new(mv);
                        alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth)//PV node so full search with no LMR
                    }else{
                        let (pvs_alpha, pvs_beta, pvs_depth) = if is_maximising{
                            (alpha, alpha+1, lmr_depth+1)
                        } else {
                            (beta-1, beta, lmr_depth+1)
                        };
                        let pvs_eval:i32 = alpha_beta_search(&current_position, pvs_alpha, pvs_beta, !is_maximising, pvs_depth, max_depth);
                        32//comment out
                    };
                }
                32//comment out
            }
        }
    }
}