use chess::Board;
pub fn evaluate(board:&Board)->i32{
    //row and col keep track of current pointer position to map the piece table values appropriately
    //piece tables are in orientation of white, so bottom rows indicate whites territory,
    // top portions indicate blacks territory

    let mut row:usize = 0;
    let mut col:usize = 0;

    let fen = board.to_string();
    let fen_parts:Vec<&str> = fen.split_whitespace().collect();
    let parse_fen = fen_parts[0];
    //ideally change this somehow,this could just be really slow...
    //takes just the string as w for denoting white to
    // move and b denoting black to move was skewing the eval

    let mut evaluation:i32 = 0;
    //actual piece tables
    let white_pawn_piece_table:[[i32;8];8] = [
        [0,   0,   0,   0,   0,   0,  0,   0],
        [98, 134,  61,  95,  68, 126, 34, -11],//black pawn place according to fen, has to be reversed to other side for symmetrical moves and proper eval
        [-6,   7,  26,  31,  65,  56, 25, -20],
        [-14,  13,   6,  21,  23,  12, 17, -23],
        [-27,  -2,  -5,  12,  17,   6, 10, -25],
        [-26,  -4,  -4, -10,   3,   3, 33, -12],
        [-35,  -1, -20, -23, -15,  24, 38, -22],//white pawn position according to fen
        [0,   0,   0,   0,   0,   0,  0,   0]];

    let black_pawn_piece_table:[[i32;8];8] = [
        [0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        [-35 ,-1 ,-20 ,-23 ,-15 ,24 ,38 ,-22 ],
        [-26 ,-4 ,-4 ,-10 ,3 ,3 ,33 ,-12 ],
        [-27 ,-2 ,-5 ,12 ,17 ,6 ,10 ,-25 ],
        [-14 ,13 ,6 ,21 ,23 ,12 ,17 ,-23 ],
        [-6 ,7 ,26 ,31 ,65 ,56 ,25 ,-20 ],
        [98 ,134 ,61 ,95 ,68 ,126 ,34 ,-11 ],
        [0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ]];

    let white_knight_piece_table:[[i32;8];8] = [
        [-167, -89, -34, -49,  61, -97, -15, -107],
        [-73, -41,  72,  36,  23,  62,   7,  -17],
        [-47,  60,  37,  65,  84, 129,  73,   44],
        [-9,  17,  19,  53,  37,  69,  18,   22],
        [-13,   4,  16,  13,  28,  19,  21,   -8],
        [-23,  -9,  12,  10,  19,  17,  25,  -16],
        [-29, -53, -12,  -3,  -1,  18, -14,  -19],
        [-105, -21, -58, -33, -17, -28, -19,  -23]];

    let black_knight_piece_table:[[i32;8];8] = [
        [-105 ,-21 ,-58 ,-33 ,-17 ,-28 ,-19 ,-23 ],
        [-29 ,-53 ,-12 ,-3 ,-1 ,18 ,-14 ,-19 ],
        [-23 ,-9 ,12 ,10 ,19 ,17 ,25 ,-16 ],
        [-13 ,4 ,16 ,13 ,28 ,19 ,21 ,-8 ],
        [-9 ,17 ,19 ,53 ,37 ,69 ,18 ,22 ],
        [-47 ,60 ,37 ,65 ,84 ,129 ,73 ,44 ],
        [-73 ,-41 ,72 ,36 ,23 ,62 ,7 ,-17 ],
        [-167 ,-89 ,-34 ,-49 ,61 ,-97 ,-15 ,-107 ]];

    let white_bishop_piece_table:[[i32;8];8] = [
        [-29,   4, -82, -37, -25, -42,   7,  -8],
        [-26,  16, -18, -13,  30,  59,  18, -47],
        [-16,  37,  43,  40,  35,  50,  37,  -2],
        [-4,   5,  19,  50,  37,  37,   7,  -2],
        [-6,  13,  13,  26,  34,  12,  10,   4],
        [0,  15,  15,  15,  14,  27,  18,  10],
        [4,  15,  16,   0,   7,  21,  33,   1],
        [-33,  -3, -14, -21, -13, -12, -39, -21]];

    let black_bishop_piece_table:[[i32;8];8] = [
        [-33 ,-3 ,-14 ,-21 ,-13 ,-12 ,-39 ,-21 ],
        [4 ,15 ,16 ,0 ,7 ,21 ,33 ,1 ],
        [0 ,15 ,15 ,15 ,14 ,27 ,18 ,10],
        [-6 ,13 ,13 ,26 ,34 ,12 ,10 ,4 ],
        [-4 ,5 ,19 ,50 ,37 ,37 ,7 ,-2 ],
        [-16 ,37 ,43 ,40 ,35 ,50 ,37 ,-2 ],
        [-26 ,16 ,-18 ,-13 ,30 ,59 ,18 ,-47 ],
        [-29 ,4 ,-82 ,-37 ,-25 ,-42 ,7 ,-8 ]];

    let white_rook_piece_table:[[i32;8];8] = [
        [32,  42,  32,  51, 63,  9,  31,  43],
        [27,  32,  58,  62, 80, 67,  26,  44],
        [-5,  19,  26,  36, 17, 45,  61,  16],
        [-24, -11,   7,  26, 24, 35,  -8, -20],
        [-36, -26, -12,  -1,  9, -7,   6, -23],
        [-45, -25, -16, -17,  3,  0,  -5, -33],
        [-44, -16, -20,  -9, -1, 11,  -6, -71],
        [-19, -13,   1,  17, 16,  7, -37, -26]];

    let black_rook_piece_table:[[i32;8];8] = [
        [-19 ,-13 ,1 ,17 ,16 ,7 ,-37 ,-26 ],
        [-44 ,-16 ,-20 ,-9 ,-1 ,11 ,-6 ,-71 ],
        [-45 ,-25 ,-16 ,-17 ,3 ,0 ,-5 ,-33 ],
        [-36 ,-26 ,-12 ,-1 ,9 ,-7 ,6 ,-23 ],
        [-24 ,-11 ,7 ,26 ,24 ,35 ,-8 ,-20 ],
        [-5 ,19 ,26 ,36 ,17 ,45 ,61 ,16 ],
        [27 ,32 ,58 ,62 ,80 ,67 ,26 ,44 ],
        [32 ,42 ,32 ,51 ,63 ,9 ,31 ,43 ]];

    let white_queen_piece_table:[[i32;8];8] = [
        [-28,   0,  29,  12,  59,  44,  43,  45],
        [-24, -39,  -5,   1, -16,  57,  28,  54],
        [-13, -17,   7,   8,  29,  56,  47,  57],
        [-27, -27, -16, -16,  -1,  17,  -2,   1],
        [ -9, -26,  -9, -10,  -2,  -4,   3,  -3],
        [-14,   2, -11,  -2,  -5,   2,  14,   5],
        [-35,  -8,  11,   2,   8,  15,  -3,   1],
        [-1, -18,  -9,  10, -15, -25, -31, -50]];

    let black_queen_piece_table:[[i32;8];8] = [
        [-1 ,-18 ,-9 ,10 ,-15 ,-25 ,-31 ,-50 ],
        [-35 ,-8 ,11 ,2 ,8 ,15 ,-3 ,1 ],
        [-14 ,2 ,-11 ,-2 ,-5 ,2 ,14 ,5 ],
        [-9 ,-26 ,-9 ,-10 ,-2 ,-4 ,3 ,-3 ],
        [-27 ,-27 ,-16 ,-16 ,-1 ,17 ,-2 ,1 ],
        [-13 ,-17 ,7 ,8 ,29 ,56 ,47 ,57 ],
        [-24 ,-39 ,-5 ,1 ,-16 ,57 ,28 ,54 ],
        [-28 ,0 ,29 ,12 ,59 ,44 ,43 ,45 ]];


    for ch in parse_fen.chars(){//the piece tables are skewing the results, so have to change them
        //write a python script to mirror the tables for black.
        // println!("char is {ch} at {row} {col}");
        match ch{
            //must be inefficient or there must be an easier way to do this, not sure
            //row and col numbers are tracked to multiply existing piece eval with piece position table

            'P' => {
                // let val = white_pawn_piece_table[row][col];
                evaluation += 100 + white_pawn_piece_table[row][col];
                col+=1;
            },
            'p' => {
                evaluation -= 100 + black_pawn_piece_table[row][col];
                col+=1;
            },
            'N' => {
                evaluation += 320 + white_knight_piece_table[row][col];
                col+=1;
            },
            'n' => {
                evaluation -= 320 + black_knight_piece_table[row][col];
                col+=1;
            },
            'B' => {
                evaluation += 330 + white_bishop_piece_table[row][col];
                col+=1;
            },
            'b' => {
                evaluation -= 330 + black_bishop_piece_table[row][col];
                col+=1;
            },
            'R' => {
                evaluation += 500 + white_rook_piece_table[row][col];
                col+=1;
            },
            'r' => {
                evaluation -= 500 + black_rook_piece_table[row][col];
                col+=1;
            },
            'Q' => {
                evaluation += 900 + white_queen_piece_table[row][col];
                col+=1;
            },
            'q' => {
                //so... queens and kings being non-symmetric
                // introduced a flaw in the logic, removing 7-col fixes the code, but id have to do that for all
                evaluation -= 900 + black_queen_piece_table[row][col];
                col+=1;
            },
            '/' => {
                //when encountering a '/' , go to the next row and make col = 0
                col = 0;
                row += 1;
            },
            'K' => {
                col+=1;
            },
            'k' => {
                col+=1;
            },
            _ => {
                // when encountering digits, it adds the digits to col. This value can't go
                // out of the bounds of 8*8, so no need for error checks yet... in-case errors arise, rectify that
                if ch.is_digit(10){
                    let val = ch.to_digit(10).unwrap() as usize;
                    col += val;
                }
                evaluation+=0;
            },
        }
    }
    evaluation
}
// add piece tables for the kings as well
//match function can be made under a single for loop efficiently