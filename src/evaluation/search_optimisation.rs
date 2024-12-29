//add in transposition tables, move ordering, quiescent search
use chess::{Board, ChessMove, MoveGen, Piece};
fn piece_value(piece:Piece) -> i16{
    match piece{
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 10000
    }
}

fn score_move(board: &Board, mv: &ChessMove) -> i16{
    let mut score:i16 = 0;
    if let Some(captured_piece) = board.piece_on(mv.get_dest()){
        let attacker = board.piece_on(mv.get_source()).unwrap();
        score += piece_value(captured_piece) * 10 - piece_value(attacker);
    }
    if board.checkers().popcnt()>0{
        score +=1000;
    }
    score
}
