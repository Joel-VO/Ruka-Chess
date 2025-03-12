//so do checks for tactical positions, checks have to have all moves, etc... pruning has to be controlled...
// add in conditions for not doing LMR in PV node and then apply this otherwise.

use arrayvec::ArrayVec;
//https://chess.stackexchange.com/questions/15856/implementing-late-move-reduction-lmr-inside-iterative-deepening
use chess::{Board, ChessMove};
use chess::Piece::Pawn;
fn tactical_position(board: &Board, moves_sorted: &ArrayVec<ChessMove, 218>) -> bool {
    for moves in moves_sorted {
        let capture = board.piece_on(moves.get_dest());

        let en_passant = if board.en_passant().is_some() {
            let dissimilar_file = (moves.get_source().get_file()) != (moves.get_dest().get_file());
            let target_capture = board.piece_on(moves.get_dest()) == None;
            let piece_start = board.piece_on(moves.get_source()) == Some(Pawn);
            piece_start && dissimilar_file && target_capture
        } else {
            false
        };

        let promotion = moves.get_promotion();
        let check_moves = board.make_move_new(*moves).checkers().popcnt() > 0;

        if capture.is_some() || en_passant || promotion.is_some() || check_moves {
            return true; // Return early if we find any tactical move
        }
    }
    false // Return false only if no tactical moves were found
}
pub fn lmr(board:&Board,moves_sorted:&ArrayVec<ChessMove,218>, depth:u8)->u8{
    if depth > 7{
        let updated_depth:f64;
        let is_tactical:bool = tactical_position(board, moves_sorted);
        if is_tactical{
            updated_depth = 0.2 + ((moves_sorted.len() as f64).ln() * (depth as f64).ln())/3.35;
        }else{
            updated_depth = 1.35 + ((moves_sorted.len() as f64).ln() * (depth as f64).ln())/2.75;
        }
        //add in logic for tactical vs quiet
        updated_depth as u8
    }else{
        depth
    }
}
