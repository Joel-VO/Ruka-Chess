use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use dashmap::DashMap;
// use crate::evaluation::evaluations::evaluate;
use crate::search::move_ordering::moves_sorted;
use crate::evaluation::evaluations::pe_sto;
use rayon::prelude::*;
use crate::search::search_improvements::zobrist_hash::updated_hash_move;
//implements parallelization
use crate::search::search_improvements::zobrist_hash::ZobristHashing;

type TranspositionTable = DashMap<u64,(i32,u8,bool)>;//maybe add in side that moved? or side to move??

pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8, start_hash:u64, zobrist_key: &ZobristHashing)->Option<ChessMove>{
    let alpha = i32::MIN;
    let beta = i32::MAX;

    let transposition_table:TranspositionTable = DashMap::new();

    //collecting possible root node moves(moves present in current position) and creates individual threads
    let legal_moves:Vec<ChessMove> = MoveGen::new_legal(&board).collect();
    let (best_move, _best_eval) = legal_moves//(best_move, eval) returns a tuple with best_move and the eval
        .par_iter()//iterates and creates a thread
        .map(//searches using alpha beta and returns the value for each root node move thread
            |&moves|{
                let updated_hash = updated_hash_move(start_hash, &moves, zobrist_key, &board);
                let current_position = board.make_move_new(moves);
                let eval = alpha_beta_search(&current_position, alpha, beta, !is_maximising, 1,max_depth, &transposition_table, updated_hash, zobrist_key);
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
    best_move
}

fn alpha_beta_search(board:&Board, mut alpha:i32, mut beta:i32, is_maximising:bool,
                     depth:u8, max_depth:u8, transposition_table:&TranspositionTable,
                     start_hash:u64,zobrist_key: &ZobristHashing) ->i32{//add in current_hash:u64
    //implement quiescent search here

    //add in condition to check transposition table for hash computed in parent
    if let Some(output) = transposition_table.get(&start_hash){//function doesn't seem to work as intended
        let (eval, hash_depth,side_to_move) = *output;
        if hash_depth > depth && !side_to_move{//apparently odd depths work and not even ones... dont know what the actual shit is going on here.
            // TT is going to be hell to debug at this rate
            return eval;
        }
    }
    if board.status() == BoardStatus::Checkmate{ //checks checkmate condition first, then draw conditions
        if board.side_to_move() == Color::White{
            -100000 + (depth as i32)
        }else{
            100000 - (depth as i32)
        }
    }else if board.status() == BoardStatus::Stalemate{
        return 0
    }else if depth == max_depth{
        return pe_sto(board);//replace with quiescent search
    }else{
        if is_maximising{
            let mut max_eval = i32::MIN;
            let legal_moves = moves_sorted(board);
            let mut first_move:bool = true;

            for mv in legal_moves{

                let hash = updated_hash_move(start_hash, &mv, zobrist_key, &board);

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth, &transposition_table, hash, zobrist_key)
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, alpha, alpha+1, false, depth+1, max_depth, &transposition_table, hash, zobrist_key);
                    if pvs_eval>alpha{
                        alpha_beta_search(&current_position, alpha, beta, false, depth+1, max_depth, &transposition_table, hash, zobrist_key)
                    }else{
                        pvs_eval
                    }
                };

                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                transposition_table.insert(hash,(max_eval,max_depth - depth,true));//logic issues
                //update hash_table
                if beta<=alpha{
                    break;
                }
            }
            max_eval


        }else{
            let mut min_eval= i32::MAX;
            let legal_moves = MoveGen::new_legal(board);
            let mut first_move = true;
            for mv in legal_moves{

                let hash = updated_hash_move(start_hash, &mv, zobrist_key, &board);//change this

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth, &transposition_table, hash,zobrist_key)
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, beta-1, beta, true, depth+1, max_depth, &transposition_table, hash,zobrist_key);
                    if pvs_eval<beta{
                        alpha_beta_search(&current_position, alpha, beta, true, depth+1, max_depth, &transposition_table,hash,zobrist_key)
                    }else{
                        pvs_eval
                    }
                };

                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                transposition_table.insert(hash,(min_eval,max_depth - depth,false));//logic issues
                if beta<=alpha{
                    break;
                }
            }
            min_eval
        }
    }
}

//add in quiescent search

// fn quiescent_search(){
//
// }
