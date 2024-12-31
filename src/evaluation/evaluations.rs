use chess::Board;
pub fn evaluate(board:&Board)->i32{
    let mut row:usize = 0;
    let mut col:usize = 0;
    let fen = board.to_string();
    let mut evaluation:i32 = 0;
    let pawn_piece_table:[[i32;8];8] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,1000,0,0,0],
        [0,0,0,0,1000,0,0,0],
        [0,0,0,0,1000,0,0,0],
        [0,0,0,0,1000,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]];
    let nightpiece_table:[[i32;8];8] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]];
    let bishoppiece_table:[[i32;8];8] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]];
    let rookpiece_table:[[i32;8];8] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]];
    let queenpiece_table:[[i32;8];8] = [
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0]];
    for ch in fen.chars(){
        if col == 8{
            col = 8
        }
        match ch{
            //must be inefficient or there must be an easier way to do this, not sure
            //row and col numbers are tracked to multiply existing piece eval with piece position table
            'P' => {
                evaluation = evaluation + (100 + pawn_piece_table[7-row][7-col]);
                row+=1;
                //
            },
            'p' => {
                evaluation = evaluation -(100 + pawn_piece_table[row][col]);
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
                if col==8{
                    return evaluation;
                }
            },
            _  => evaluation+=0,
        }
    }
    evaluation
}
// add piece tables for the kings as well, after ensuring that the piece table is working.
