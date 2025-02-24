use chess::{Board, ALL_SQUARES, Color, File, Square, Rank, Piece};
use crate::evaluation::piece_square_tables::eg_piece_square_table::EG_PIECE_SQUARE_TABLES;
use crate::evaluation::piece_square_tables::mg_piece_square_table::MG_PIECE_SQUARE_TABLES;
// pub fn pe_sto(board: &Board) -> i32{
//     let mg_value:[i32;6] = [82, 337, 365, 477, 1025,  0];
//     let eg_value:[i32;6] = [94, 281, 297, 512,  936,  0];
//     let game_phase_inc:[i32;12] = [0,0,1,1,1,1,2,2,4,4,0,0];//adjust weights to be powers of 2, so bitwise division can be done.
//     let board_fen = board.to_string();
//     let split_fen:Vec<&str> = board_fen.split_whitespace().collect();
//     let fen = split_fen[0];
//     //implement piece square_tables.
//
//     let mut mg:[i32;2] = [0,0];//0 for white, 1 for black, this is the middle game table
//     let mut eg:[i32;2] = [0,0];//endgame table
//
//     let mut col:usize = 0;
//
//     let mut game_phase = 0;
//
//     for char in fen.chars(){//replace match with Square::ALL and loop through squares. much faster.
//         match char{
//             'P' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[0][col] + mg_value[0];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[0][col] + eg_value[0];
//                 col+=1;
//                 game_phase += game_phase_inc[0];
//             }
//             'p'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[1][col] + mg_value[0];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[1][col] + eg_value[0];
//                 col+=1;
//                 game_phase += game_phase_inc[1];
//             }
//             'N' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[2][col] + mg_value[1];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[2][col] + eg_value[1];
//                 col+=1;
//                 game_phase += game_phase_inc[2];
//             }
//             'n'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[3][col] + mg_value[1];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[3][col] + eg_value[1];
//                 col+=1;
//                 game_phase += game_phase_inc[3];
//             }
//             'B' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[4][col] + mg_value[2];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[4][col] + eg_value[2];
//                 col+=1;
//                 game_phase += game_phase_inc[4];
//             }
//             'b'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[5][col] + mg_value[2];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[5][col] + eg_value[2];
//                 col+=1;
//                 game_phase += game_phase_inc[5];
//             }
//             'R' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[6][col] + mg_value[3];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[6][col] + eg_value[3];
//                 col+=1;
//                 game_phase += game_phase_inc[6];
//             }
//             'r'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[7][col] + mg_value[3];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[7][col] + eg_value[3];
//                 col+=1;
//                 game_phase += game_phase_inc[7];
//             }
//             'Q' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[8][col] + mg_value[4];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[8][col] + eg_value[4];
//                 col+=1;
//                 game_phase += game_phase_inc[8];
//             }
//             'q'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[9][col] + mg_value[4];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[9][col] + eg_value[4];
//                 col+=1;
//                 game_phase += game_phase_inc[9];
//             }
//             'K' => {
//                 mg[0] += MG_PIECE_SQUARE_TABLES[10][col] + mg_value[5];
//                 eg[0] += EG_PIECE_SQUARE_TABLES[10][col] + eg_value[5];
//                 col+=1;
//                 game_phase += game_phase_inc[10];
//             }
//             'k'=> {
//                 mg[1] += MG_PIECE_SQUARE_TABLES[11][col] + mg_value[5];
//                 eg[1] += EG_PIECE_SQUARE_TABLES[11][col] + eg_value[5];
//                 col+=1;
//                 game_phase += game_phase_inc[11];
//             }
//             _ =>{
//                 if char.is_digit(10){
//                     let step_col = char.to_digit(10).unwrap() as usize;
//                     col += step_col;
//                 }
//             }
//         }
//     }
//     //tapered evaluation
//     let mg_score = mg[0] - mg[1];
//     let eg_score = eg[0] - eg[1];
//     let mg_phase = {
//         if game_phase > 24{
//             24
//         }else{
//             game_phase
//         }
//     };
//     let eg_phase = 24-mg_phase;
//     let eval:i32 = (mg_score*mg_phase + eg_score*eg_phase)/24;
//
//     //add in king safety and pawn structure ideas
//
//     eval
// }


pub fn evaluation_func(board: &Board) -> i32{
    const MG_VALUES:[i32;6] = [82, 337, 365, 477, 1025,  0];
    const EG_VALUES:[i32;6] = [94, 281, 297, 512,  936,  0];
    const GAME_PHASE_INC:[i32;12] = [0,0,1,1,1,1,2,2,4,4,0,0];//adjust weights to be powers of 2, so bitwise division can be done.

    const TABLE_INDEX:[[usize;2];6] = [
        [0, 1],
        [2, 3],
        [4, 5],
        [6, 7],
        [8, 9],
        [10, 11],
    ];

    let mut mg:[i32;2] = [0,0];//0 for white, 1 for black, this is the middle game table
    let mut eg:[i32;2] = [0,0];//endgame table

    let mut game_phase = 0;

    for sq in ALL_SQUARES.iter(){
        if let Some(piece) = board.piece_on(*sq){
            let color_index = board.color_on(*sq).unwrap().to_index();
            let piece_index = piece.to_index();
            let table_index = TABLE_INDEX[piece_index][color_index];
            let sq_index = sq.to_index();
            mg[color_index] += MG_PIECE_SQUARE_TABLES[table_index][63-sq_index] + MG_VALUES[piece_index];
            eg[color_index] += EG_PIECE_SQUARE_TABLES[table_index][63-sq_index] + EG_VALUES[piece_index];
            game_phase += GAME_PHASE_INC[table_index];
        }
    }
    let mg_score = mg[0] - mg[1];
    let eg_score = eg[0] - eg[1];
    let mg_phase = game_phase.min(24);
    let eg_phase = 24-mg_phase;
    let eval:i32 = (mg_score*mg_phase + eg_score*eg_phase)/24;
    let complete_eval = additional_eval_capability(board,eval, mg_phase);
    complete_eval
}

pub fn additional_eval_capability(board:&Board,eval:i32, game_phase:i32) -> i32{
    //king-side safety
    let mut resultant_eval:i32 = eval;
    let color:Color = board.side_to_move();
    let king_square = board.king_square(color);
    let king_file = king_square.get_file();
    let castled = if king_file == File::C || king_file == File::G{
        let rank_val =  if color == Color::White{Rank::First} else {Rank::Eighth};

        let rook_king = Square::make_square(rank_val, File::F);
        let rook_queen = Square::make_square(rank_val, File::D);

        if let Some(rook) = board.piece_on(rook_king){
            if rook == Piece::Rook{
                true
            }else{
                false
            }
        }else if let Some(rook) = board.piece_on(rook_queen){
            if rook == Piece::Rook{
                true
            }else{
                false
            }
        }else{
            false
        }
    }else{
        false
    };
    if castled && game_phase > 10{
        resultant_eval += 100;
    }
    resultant_eval
}