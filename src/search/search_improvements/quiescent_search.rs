//add in quiescent search to negate the effects of the horizon effect
//will increase search times, but not much and moves will be better as critical captures are looked further into, improving the search result.
//not all nodes are looked into so not too expensive

use chess::{Board, ChessMove, MoveGen};
use crate::evaluation::evaluations::pe_sto;

fn tactical_moves(board: &Board)->Vec<ChessMove>{
    ///returns a Vec<ChessMove> of all legal captures in a position. Can be modified to add checks as well to improve search quality.
    let move_gen = MoveGen::new_legal(board);
    let mut avail_moves:Vec<ChessMove> = Vec::new();
    for moves in move_gen {
        if board.piece_on(moves.get_dest()).is_some() {
            avail_moves.push(moves);
        }
    }
    avail_moves
}

pub fn quiescent_search(board: &Board, mut alpha:i32, mut beta:i32, depth:u8, max_depth:u8, is_maximising:bool) ->i32{

    let moves_tactical = tactical_moves(board);

    if depth >= max_depth || moves_tactical.len()==0{
        //greater than or equal to, to prevent un stoppable search extension in case of some error.
        // This part was overlooked in the CPW pseudocode(according to what i read) and this is necessary to prevent moves that are egregious...
        return pe_sto(board);
    }

    let stand_pat:i32 = pe_sto(board);

    //https://www.chessprogramming.org/Quiescence_Search

    if is_maximising{//performing standard alpha beta init
        if stand_pat >= beta{
            return stand_pat //fail-soft and fail hard condition
        }
        if stand_pat > alpha{
            alpha = stand_pat
        }
    }else{
        // reverses all that's done in the above statements... as negamax pseudocode was given in CPW.
        // the reverse should work for regular alpha beta... as it does according to this code...
        // the exact workings are beyond me but got the general idea of a search window using stand_pat and beta as a window.
        if stand_pat <= alpha{
            return stand_pat
        }
        if stand_pat < beta{
            beta = stand_pat
        }
    }


    let mut best_val = if is_maximising{
        // val init for best_val. Standard alpha beta style
        i32::MIN
    }else{
        i32::MAX
    };

    for moves in moves_tactical{
        // println!("{moves}");
        let current_position = board.make_move_new(moves);
        let eval:i32 = quiescent_search(&current_position, alpha, beta, depth+1, max_depth, !is_maximising);
        if is_maximising{//alpha beta updation
            best_val = best_val.max(eval);
            alpha = alpha.max(eval);
        }else{
            best_val = best_val.min(eval);
            beta = beta.min(eval);
        }
        if beta<=alpha{
            break
        }
    }
    best_val
}
