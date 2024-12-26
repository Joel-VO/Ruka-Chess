use chess::{Board, BoardStatus, Color, MoveGen};
use crate::evaluation::piece_count_eval::evaluate;

mod evaluation;
fn main(){
    let board = Board::default();
    let eval:f32 = evaluate(&board);
    println!("{eval}")
}
fn alpha_beta_search(board:&Board, depth:u8, mut alpha:f32, mut beta:f32, is_maximising:bool, max_depth:u8)->f32{
    if board.status() == BoardStatus::Checkmate{

        if board.side_to_move() == Color::White{
            20000.0
        }else{
            -20000.0
        }
    }else if board.status() == BoardStatus::Stalemate {
        return 0.0
    }else if depth == max_depth{
        return evaluate(board);
    }else{
        if is_maximising{
            let mut max_eval:f32 = f32::MIN;
            let legal_moves = MoveGen::new_legal(board);
            for mv in legal_moves{
                let mut current_position = board.clone();
                current_position.make_move(mv, &mut current_position);//problem
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
            let mut min_eval:f32 = f32::MAX;
            let legal_moves = MoveGen::new_legal(board);
            for mv in legal_moves{
                let mut current_position = board.clone();
                current_position.make_move(mv, &mut current_position);//problem
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