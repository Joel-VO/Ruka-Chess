use chess::{Board, BoardStatus, ChessMove, Color};
// use dashmap::DashMap;
use crate::search::move_ordering::moves_sorted;
use rayon::prelude::*;//implements parallelization
use crate::search::search_improvements::quiescent_search::q_search;

// type TranspositionTable = DashMap<u64,(i32,u8)>;

pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8)->Option<(ChessMove, i32)>{
    let alpha = i32::MIN;
    let beta = i32::MAX;

    //collecting possible root node moves(moves present in current position) and creates individual threads
    let legal_moves:Vec<ChessMove> = moves_sorted(board,0,true);
    let (best_move, _best_eval) = legal_moves//(best_move, eval) returns a tuple with best_move and the eval
        .par_iter()//iterates and creates a thread
        .map(//searches using alpha beta and returns the value for each root node move thread
            |&moves|{
                let current_position = board.make_move_new(moves);
                let eval = alpha_beta_search(&current_position, alpha, beta, !is_maximising, 1,max_depth,true);//not sure of the true here
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
                     depth:u8, max_depth:u8,pv_node:bool) ->i32{//add in current_hash:u64

    //add in condition to check transposition table for hash computed in parent
    if board.status() == BoardStatus::Checkmate{ //checks checkmate condition first, then draw conditions
        if board.side_to_move() == Color::White{
            -400000 + (depth as i32)
        }else{
            400000 - (depth as i32)
        }
    }else if board.status() == BoardStatus::Stalemate{
        return 0
    }else if depth == max_depth{
        // return pe_sto(board);//replace with quiescent search
        return q_search(board, alpha, beta, depth, depth+3, is_maximising)
    }else{
        if is_maximising{
            let mut max_eval = i32::MIN;
            let mut first_move:bool = true;
            let legal_moves = moves_sorted(board,depth,pv_node);


            for mv in legal_moves{

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth,true)//PV node so full search with no LMR
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, alpha, alpha+1, false, depth+1, max_depth,false);//PVS with LMR
                    if pvs_eval >= beta{
                        pvs_eval
                    }else if pvs_eval > alpha{
                        alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth,true)//full search, no LMR
                    }else{
                        pvs_eval
                    }
                };

                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);

                //update hash_table
                if beta<=alpha{
                    break;
                }
            }
            max_eval


        }else{
            let mut min_eval= i32::MAX;
            let legal_moves = moves_sorted(board, depth,pv_node);//added depth
            let mut first_move = true;
            for mv in legal_moves{


                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth,true)//PVS with no LMR
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, beta-1, beta, true, depth+1, max_depth,false);//PVS with LMR
                    if pvs_eval <= alpha{
                        pvs_eval
                    }else if pvs_eval < beta{
                        alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth,true)//full search with LMR removed
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