mod evaluation;

use std::str::FromStr;
use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::evaluation::piece_count_eval::evaluate;

fn main(){
    let fen = "4rb1k/2pqn2p/6pn/ppp3N1/P1QP2b1/1P2p3/2B3PP/B3RRK1 w - - 0 24";
    match Board::from_str(fen){
        Ok(board) => {
            if let Some(mov) = best_move(&board, true, 5){
                println!("best move is {mov}");
            }else{
                println!("No moves available");
            }
        }
        Err(err) => {
            println!("error {err}");
        }
    }
}

fn alpha_beta_search(board:&Board, depth:u8, mut alpha:f32, mut beta:f32, is_maximising:bool, max_depth:u8)->f32{
    if board.status() == BoardStatus::Checkmate{
        if board.side_to_move() == Color::White{
            -20000.0
        }else{
            20000.0
        }
    }else if board.status() == BoardStatus::Stalemate {
        return 0.0
    }else if depth == max_depth{
        return evaluate(board);
    }else{
        if is_maximising{
            let mut max_eval:f32 = f32::NEG_INFINITY;
            let legal_moves = MoveGen::new_legal(board);
            for mv in legal_moves{
                let current_position = board.make_move_new(mv);//?
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
            let mut min_eval:f32 = f32::INFINITY;
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
fn best_move(board:&Board, is_maximising:bool, max_depth:u8)->Option<ChessMove>{
    let mut best_move = None;
    let mut best_eval = if is_maximising {
        f32::NEG_INFINITY
    } else {
        f32::INFINITY
    };
    let alpha = f32::NEG_INFINITY;
    let beta = f32::INFINITY;
    let legal_moves = MoveGen::new_legal(&board);
    for moves in legal_moves{
        let current_position = board.make_move_new(moves);//?
        let eval = alpha_beta_search(&current_position, 0, alpha, beta, !is_maximising, max_depth);
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
