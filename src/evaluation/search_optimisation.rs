//add in transposition tables, move ordering, quiescent search
use chess::{Board, ChessMove, MoveGen, Piece};
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
fn score_move(board: &Board, mv: &ChessMove) -> i16{
    let mut score:i16 = 0;
    if let Some(captured_piece) = board.piece_on(mv.get_dest()){//MVV-LVA sorting
        let attacker = board.piece_on(mv.get_source()).unwrap();
        score += piece_value(captured_piece) * 10 - piece_value(attacker);
    }

    {//to make temp_board go out of scope once used.
        let temp_board = board.make_move_new(*mv);
        if temp_board.checkers().popcnt()>0{
            score +=100;
        }
    }

    if mv.get_promotion().is_some(){// can be fine-tuned
        score+=50;
    }
    score
}
pub fn moves_sorted(board:&Board){//this has to be a sorted array of possible moves... has to have a return function
    let moves = MoveGen::new_legal(board);
}
