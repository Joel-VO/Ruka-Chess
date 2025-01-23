//add in quiescent search to negate the effects of the horizon effect
//will increase search times, but not much and moves will be better as critical captures are looked further into, improving the search result.
//not all nodes are looked into so not too expensive

use chess::{Board, ChessMove, MoveGen};
use crate::evaluation::evaluations::pe_sto;

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

pub fn quiescent_search(board: &Board, mut alpha:i32, beta:i32, depth:u8, max_depth:u8) ->i32{//not working as intended... this is
    // right now performing like a standard pesto eval.
    if depth >= max_depth{//greater than or equal to, to prevent un stoppable search extension in case of some error.
        return pe_sto(board);
    }
    let stand_pat:i32 = pe_sto(board);
    let mut best_val:i32 = stand_pat;
    if stand_pat >= beta{
        return stand_pat;
    }
    if stand_pat > alpha{
        alpha = stand_pat;
    }
    let moves_tactical = tactical_moves(board);
    for moves in moves_tactical{
        println!("{moves}");
        let current_position = board.make_move_new(moves);
        let eval:i32 = quiescent_search(&current_position, -beta, -alpha, depth+1, max_depth);
        if eval>=beta{
            return eval;
        }
        if eval > best_val{
            best_val = eval;
        }
        if eval > alpha{
            alpha = eval;
        }
    }
    best_val
}
