//not sure if this is a good idea at all...
//LMP can backfire badly…


//so do checks for tactical positions, checks have to have all moves, etc... pruning has to be controlled...
// add in conditions for not doing LMR in PV node and then apply this otherwise.

//https://chess.stackexchange.com/questions/15856/implementing-late-move-reduction-lmr-inside-iterative-deepening
//refer this
//has to be tuned down a lot as 5 is very aggressive...
// add in LMR, reducing depth searched
use chess::{Board, ChessMove};
use chess::Piece::Pawn;
fn tactical_position(board:&Board, moves_sorted:&Vec<ChessMove>) -> bool{//call this function and test this out !!!!!!!!!!!!!!!!!!
    //returns a Vec<ChessMove> of all legal captures in a position. Can be modified to add checks as well to improve search quality.
    let mut is_tactical:bool = false;
    for moves in moves_sorted{
        let capture = board.piece_on(moves.get_dest());

        let en_passant:bool = if board.en_passant().is_some(){
            let dissimilar_file:bool = (moves.get_source().get_file()) != (moves.get_dest().get_file());
            let target_capture:bool = board.piece_on(moves.get_dest()) == None;
            let piece_start:bool = board.piece_on(moves.get_source()) == Some(Pawn);
            piece_start && dissimilar_file && target_capture
        }else{
            false
        };
        let promotion = moves.get_promotion();
        let check_moves:bool = if board.make_move_new(*moves).checkers().popcnt()>0{
            true
        }else{
            false
        };

        if capture.is_some() || en_passant || promotion.is_some() || check_moves{
            is_tactical = true;
        }else{
            is_tactical = false;
        }
    }
    is_tactical
}
pub fn lmr(board:&Board,moves_sorted:&Vec<ChessMove>, depth:u8)->u8{
    if depth > 3{
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
//add in conditions for checks, etc. this has to be carefully handled or the engine will make erroneous blunders down the line

// fn null_move_pruning{
//
// }
// Weiss reduces by 0.20 + ln(depth) * ln(move number) / 3.35 for captures and promotions and 1.35 + ln(depth) * ln(move number) / 2.75 for quiet moves.
// Ethereal reduces by 0.7844 + ln(depth) * ln(moves played) / 2.4696 for quiet moves and 3 (or 2 if the move gave check) for captures and promotions
// Senpai reduces by one ply for the first 6 moves and by depth / 3 for remaining moves.
// Fruit Reloaded uses formula: uint8( sqrt(double(depth-1)) + sqrt(double(moves-1))); for non-PV nodes. In PV-nodes it reduces by 2/3 of this value.