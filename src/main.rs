use chess::{Board, MoveGen};
mod evaluation;
fn main(){
    let board = Board::default();
    let eval:f32 = evaluation::piece_count_eval::evaluate(&board);
    println!("{eval}")
}
