use chess::{Board, ChessMove, MoveGen, Piece};
use crate::search::search_improvements::lmp::lm_pruning;
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
pub fn moves_sorted(board:&Board, depth:u8, pv_node:bool) -> Vec<ChessMove> { // sorted array of possible moves
    let move_gen = MoveGen::new_legal(board);
    let mut moves: Vec<ChessMove> = move_gen.collect();
    //ideally convert this to an array to improve speed...issue is size has to be fixed, but that's
    // not an issue, the sorting has to be done using merge sort or similar algorithm to speed up sorting
    moves.sort_by_key(|mv| -score_move(board, mv)); //descending order

    //so do checks for tactical positions, checks have to have all moves, etc etc... pruning has to be controlled...
    // add in conditions for not doing LMR in PV node and then apply this otherwise.

    //https://chess.stackexchange.com/questions/15856/implementing-late-move-reduction-lmr-inside-iterative-deepening
    //refer this
    if pv_node{
       moves
    }else{
        let reduced_moves = lm_pruning(moves, depth);
        reduced_moves
    }
}
//make move ordering faster by eliminating the need for the vectors. use an array with max size 218(theoretical max of the number of moves...)