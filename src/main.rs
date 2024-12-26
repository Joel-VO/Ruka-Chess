use chess::{Board, MoveGen};

fn main(){
    let board = Board::default();
    let eval = evaluate(&board);
    println!("{eval}")
}
fn evaluate(board:&Board)->f32{
    let fen = board.to_string();
    let mut evaluation:f32 = 0.0;
    for ch in fen.chars(){
        match ch{
            'P' => evaluation+=100.0,
            'p' => evaluation+=-100.0,
            'N' => evaluation+=320.0,
            'n' => evaluation+=-320.0,
            'B' => evaluation+=330.0,
            'b' => evaluation+=-330.0,
            'R' => evaluation+=500.0,
            'r' => evaluation+=-500.0,
            'Q' => evaluation+=900.0,
            'q' => evaluation+=-900.0,
            _  => evaluation+=0.0,
        }
    }
    evaluation
}