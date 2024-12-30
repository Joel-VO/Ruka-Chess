use chess::Board;
pub fn evaluate(board:&Board)->i32{
    let mut row:u8 = 0;
    let mut col:u8 = 0;
    let fen = board.to_string();
    let mut evaluation:i32 = 0;
    for ch in fen.chars(){
        match ch{
            //must be inefficient or there must be an easier way to do this, not sure
            //row and col numbers are tracked to multiply existing piece eval with piece position table
            'P' => {
                evaluation+=100;
                row+=1;
            },
            'p' => {
                evaluation += -100;
                row+=1;
            },
            'N' => {
                evaluation += 320;
                row+=1;
            },
            'n' => {
                evaluation += -320;
                row+=1;
            },
            'B' => {
                evaluation += 330;
                row+=1;
            },
            'b' => {
                evaluation += -330;
                row+=1;
            },
            'R' => {
                evaluation += 500;
                row+=1;
            },
            'r' => {
                evaluation += -500;
                row+=1;
            },
            'Q' => {
                evaluation += 900;
                row+=1;
            },
            'q' => {
                evaluation += -900;
                row+=1;
            },
            '/' => {
                row = 0;
                col+=1;
            },
            _  => evaluation+=0,
        }
    }
    evaluation
}
//adding piece table concepts.
//add an array of position values for each piece.
