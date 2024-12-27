use chess::Board;
pub fn evaluate(board:&Board)->i32{
    let fen = board.to_string();
    let mut evaluation:i32 = 0;
    for ch in fen.chars(){
        match ch{
            'P' => evaluation+=100,
            'p' => evaluation+=-100,
            'N' => evaluation+=320,
            'n' => evaluation+=-320,
            'B' => evaluation+=330,
            'b' => evaluation+=-330,
            'R' => evaluation+=500,
            'r' => evaluation+=-500,
            'Q' => evaluation+=900,
            'q' => evaluation+=-900,
            _  => evaluation+=0,
        }
    }
    evaluation
}