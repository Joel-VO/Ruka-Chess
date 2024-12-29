use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::evaluations::evaluate;

pub fn best_move(board:&Board, is_maximising:bool, max_depth:u8)->Option<ChessMove>{
    let mut best_move = None;
    let mut best_eval = if is_maximising {
        i32::MIN
    } else {
        i32::MAX
    };
    let alpha = i32::MIN;
    let beta = i32::MAX;
    let legal_moves = MoveGen::new_legal(&board);
    for moves in legal_moves{
        let current_position = board.make_move_new(moves);
        let eval = alpha_beta_search(&current_position, 1, alpha, beta, !is_maximising, max_depth);
        if is_maximising && (eval>best_eval){
            best_move = Some(moves);
            best_eval = eval;
        }else if !is_maximising && (eval<best_eval){
            best_move = Some(moves);
            best_eval = eval;
        }
    }
    best_move
}
fn alpha_beta_search(board:&Board, depth:u8, mut alpha:i32, mut beta:i32, is_maximising:bool, max_depth:u8) ->i32{
    if board.status() == BoardStatus::Checkmate{
        if board.side_to_move() == Color::White{
            -20000
        }else{
            20000
        }
    }else if board.status() == BoardStatus::Stalemate {
        return 0
    }else if depth == max_depth{
        return evaluate(board);
    }else{
        if is_maximising{
            let mut max_eval = i32::MIN;
            let legal_moves = MoveGen::new_legal(board);
            for mv in legal_moves{
                let current_position = board.make_move_new(mv);
                let eval = alpha_beta_search(&current_position, depth+1, alpha, beta, false, max_depth);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta<=alpha{
                    break;
                }
            }
            max_eval
        }else{
            //create minimizing code
            let mut min_eval= i32::MAX;
            let legal_moves = MoveGen::new_legal(board);
            for mv in legal_moves{
                let current_position = board.make_move_new(mv);//?
                let eval = alpha_beta_search(&current_position, depth+1, alpha, beta, true, max_depth);
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
