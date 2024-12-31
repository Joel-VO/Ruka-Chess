use chess::Board;
pub fn evaluate(board:&Board)->i32{
    let mut row:usize = 0;
    let mut col:usize = 0;
    let fen = board.to_string();
    let fen_parts:Vec<&str> = fen.split_whitespace().collect();
    let parse_fen = fen_parts[0];
    let mut evaluation:i32 = 0;

    let pawn_piece_table:[[i32;8];8] = [
        [0,   0,   0,   0,   0,   0,  0,   0],
        [98, 134,  61,  95,  68, 126, 34, -11],
        [-6,   7,  26,  31,  65,  56, 25, -20],
        [-14,  13,   6,  21,  23,  12, 17, -23],
        [-27,  -2,  -5,  12,  17,   6, 10, -25],
        [-26,  -4,  -4, -10,   3,   3, 33, -12],
        [-35,  -1, -20, -23, -15,  24, 38, -22],
        [0,   0,   0,   0,   0,   0,  0,   0]];
    let val = pawn_piece_table[6][0];
    println!("val is{val}");
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
    for ch in parse_fen.chars(){
        match ch{
            //must be inefficient or there must be an easier way to do this, not sure
            //row and col numbers are tracked to multiply existing piece eval with piece position table
            'P' => {
                let mut val = pawn_piece_table[row][col];
                evaluation += 100 + val;
                println!("eval for white pawn reached...{row}{col} and val is {val}");
                col+=1;
                //
            },
            'p' => {
                let mut val = pawn_piece_table[7-row][7-col];
                evaluation +=  -(100 + val);
                println!("eval for black pawn reached...{row}{col} and val is {val}");
                col+=1;
                //
            },
            'N' => {
                evaluation += 320;
                col+=1;
            },
            'n' => {
                evaluation += -320;
                col+=1;
            },
            'B' => {
                evaluation += 330;
                col+=1;
            },
            'b' => {
                evaluation += -330;
                col+=1;
            },
            'R' => {
                evaluation += 500;
                col+=1;
            },
            'r' => {
                evaluation += -500;
                col+=1;
            },
            'Q' => {
                evaluation += 900;
                col+=1;
            },
            'q' => {
                evaluation += -900;
                col+=1;
            },
            '/' => {
                col = 0;
                row+=1;
            },
            _  => {
                evaluation+=0;
            },
        }
    }
    evaluation
}
// add piece tables for the kings as well, after ensuring that the piece table is working.
