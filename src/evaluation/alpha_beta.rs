use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::evaluations::evaluate;
use crate::evaluation::move_ordering::moves_sorted;
use rayon::prelude::*;//implements parallelization
pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8)->Option<ChessMove>{
    let alpha = i32::MIN;
    let beta = i32::MAX;

    //collecting possible root node moves(moves present in current position) and creates individual threads
    let legal_moves:Vec<ChessMove> = MoveGen::new_legal(&board).collect();
    let (best_move, _best_eval) = legal_moves//(best_move, eval) returns a tuple with best_move and the eval
        .par_iter()//iterates and creates a thread
        .map(//searches using alpha beta and returns the value for each root node move thread
            |&moves|{
                let current_position = board.make_move_new(moves);
                let eval = alpha_beta_search(&current_position, alpha, beta, !is_maximising, max_depth);
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

fn alpha_beta_search(board:&Board, mut alpha:i32, mut beta:i32, is_maximising:bool, depth:u8) ->i32{
    //add in Principal variance search to speed up searching
    //to be done after move ordering or else engine will perform worse.
    //implement quiescent search here
    if board.status() == BoardStatus::Checkmate{//checks checkmate condition first, then draw conditions
        if board.side_to_move() == Color::White{
            -20000
        }else{
            20000
        }
    }else if board.status() == BoardStatus::Stalemate {
        return 0
    }else if depth == 0{
        return evaluate(board);
    }else{
        if is_maximising{
            let mut max_eval = i32::MIN;
            let legal_moves = moves_sorted(board);
            let mut first_move:bool = true;
            for mv in legal_moves{

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, false, depth-1)
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, alpha, alpha+1, false, depth-1);
                    if pvs_eval>alpha{
                        alpha_beta_search(&current_position, alpha, beta, false, depth-1)
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
            let legal_moves = MoveGen::new_legal(board);
            let mut first_move = true;
            for mv in legal_moves{

                let eval = if first_move{
                    first_move=false;
                    let current_position:Board = board.make_move_new(mv);
                    alpha_beta_search(&current_position, alpha, beta, true, depth-1)
                }else{
                    let current_position:Board = board.make_move_new(mv);
                    let pvs_eval = alpha_beta_search(&current_position, beta-1, beta, true, depth-1);
                    if pvs_eval<beta{
                        alpha_beta_search(&current_position, alpha, beta, true, depth-1)
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
//add in quiescent search
//sometimes, engine doesn't do the mate. some bug in the code

// fn quiescent_search(){
//
// }