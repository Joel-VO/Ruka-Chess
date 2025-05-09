use chess::{Board, ChessMove, MoveGen, Piece};
use arrayvec::ArrayVec;
fn piece_value(piece:Piece) -> i16{
    match piece{//values have to be fine-tuned
        Piece::Pawn => 10,
        Piece::Knight => 32,
        Piece::Bishop => 33,
        Piece::Rook => 50,
        Piece::Queen => 90,
        Piece::King => 10000
    }
}

fn score_move(board: &Board, mv: &ChessMove) -> i16{//prioritise checks and captures
    //score stores the score of a move
    let mut score:i16 = 0;
    if let Some(captured_piece) = board.piece_on(mv.get_dest()){//MVV-LVA sorting
        let attacker = board.piece_on(mv.get_source()).unwrap();
        score += piece_value(captured_piece) * 100 - piece_value(attacker);
    }

    {//to make temp_board go out of scope once used.
        let temp_board = board.make_move_new(*mv);
        if temp_board.checkers().popcnt()>0{
            score +=1000;
        }
    }

    if mv.get_promotion().is_some(){// can be fine-tuned
        score+=1000;
    }
    score
}
pub fn moves_sorted(board:&Board) -> ArrayVec<ChessMove,218> { // sorted array of possible moves
    let move_gen = MoveGen::new_legal(board);
    // let mut moves: Vec<ChessMove> = move_gen.collect();
    let mut moves = ArrayVec::<ChessMove,218>::new();
    moves.extend(move_gen);
    moves.sort_by_key(|mv| -score_move(board, mv)); //descending order
    moves
}
//make move ordering faster by eliminating the need for the vectors. use an array with max size 218(theoretical max of the number of moves...)