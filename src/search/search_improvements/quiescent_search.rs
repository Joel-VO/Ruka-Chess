//add in quiescent search to negate the effects of the horizon effect
//will increase search times, but not much and moves will be better as critical captures are looked further into, improving the search result.
//not all nodes are looked into so not too expensive

use chess::{Board, ChessMove, MoveGen};
use crate::evaluation::evaluations::pe_sto;

pub fn quiescent_search(board: &Board, mut alpha:i32, beta:i32) ->i32{
    let stand_pat:i32 = pe_sto(board);
    let best_val:i32 = stand_pat;
    if stand_pat >= beta{
        return stand_pat
    }
    if stand_pat > alpha{
        alpha = stand_pat;
    }
    let moves_tactical = tactical_moves(board);
    for moves in moves_tactical{

    }
    32
}

fn tactical_moves(board: &Board)->Vec<ChessMove>{
    let move_gen = MoveGen::new_legal(board);
    let mut avail_moves:Vec<ChessMove> = Vec::new();
    for moves in move_gen {
        if board.piece_on(moves.get_dest()).is_some() {
            avail_moves.push(moves);
        }
    }
    avail_moves
}


//CPW pseudo code

// int Quiesce( int alpha, int beta ) {
// int stand_pat = Evaluate();
// int best_value = stand_pat;
// if( stand_pat >= beta )
// return stand_pat;
// if( alpha < stand_pat )
// alpha = stand_pat;
//
// until( every_capture_has_been_examined )  {
// MakeCapture();
// score = -Quiesce( -beta, -alpha );
// TakeBackMove();
//
// if( score >= beta )
// return score;
// if( score > best_value )
// best_value = score;
// if( score > alpha )
// alpha = score;
// }
//
// return best_value;
// }