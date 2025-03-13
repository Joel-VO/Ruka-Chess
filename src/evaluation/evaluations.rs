use chess::{Board, ALL_SQUARES};// Color, File, Square, Rank, Piece
use crate::evaluation::piece_square_tables::eg_piece_square_table::EG_PIECE_SQUARE_TABLES;
use crate::evaluation::piece_square_tables::mg_piece_square_table::MG_PIECE_SQUARE_TABLES;

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
            mg[color_index] += MG_PIECE_SQUARE_TABLES[table_index][sq_index] + MG_VALUES[piece_index];
            eg[color_index] += EG_PIECE_SQUARE_TABLES[table_index][sq_index] + EG_VALUES[piece_index];
            game_phase += GAME_PHASE_INC[table_index];
        }
    }
    let mg_score = mg[0] - mg[1];
    let eg_score = eg[0] - eg[1];
    let mg_phase = game_phase.min(24);
    let eg_phase = 24-mg_phase;
    let eval:i32 = (mg_score*mg_phase + eg_score*eg_phase)/24;
    // let complete_eval = additional_eval_capability(board,eval, mg_phase);
    // complete_eval
    eval
}

// fn additional_eval_capability(board:&Board,eval:i32, game_phase:i32) -> i32{
//     //king-side safety
//     let mut resultant_eval:i32 = eval;
//     let color:Color = board.side_to_move();
//     let king_square = board.king_square(color);
//     let king_file = king_square.get_file();
//     let castled = if king_file == File::C || king_file == File::G{
//         let rank_val =  if color == Color::White{Rank::First} else {Rank::Eighth};
//
//         let rook_king = Square::make_square(rank_val, File::F);
//         let rook_queen = Square::make_square(rank_val, File::D);
//
//         if let Some(rook) = board.piece_on(rook_king){
//             if rook == Piece::Rook{
//                 true
//             }else{
//                 false
//             }
//         }else if let Some(rook) = board.piece_on(rook_queen){
//             if rook == Piece::Rook{
//                 true
//             }else{
//                 false
//             }
//         }else{
//             false
//         }
//     }else{
//         false
//     };
//     if castled && game_phase > 12{
//         resultant_eval += 50;
//     }
//
//     //add in doubled pawn logic later and other positional concepts
//
//
//     resultant_eval
// }
//https://github.com/official-stockfish/nnue-pytorch/blob/master/docs/nnue.md
//github link explaining NNUE