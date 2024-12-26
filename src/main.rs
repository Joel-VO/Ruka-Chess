use chess::{Board, BoardStatus, Color, MoveGen};
mod evaluation;
fn main(){
    let board = Board::default();
    let eval:f32 = evaluation::piece_count_eval::evaluate(&board);
    println!("{eval}")
}
fn alpha_beta_search(board:Board, depth:u8, alpha:f64, beta:f64, is_maximising:bool, max_depth:u8)->f32{
    if board.status() == BoardStatus::Checkmate{

        if board.side_to_move() == Color::White{
            20000.0
        }else{
            -20000.0
        }

    }else if board.status() == BoardStatus::Stalemate {
        return 0.0
    }else{
        if is_maximising{
            //create maximising code
        }else{
            //create minimizing code
        }
    }
}