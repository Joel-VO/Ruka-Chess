//not sure if this is a good idea at all...
//LMP can backfire badly..


//so do checks for tactical positions, checks have to have all moves, etc etc... pruning has to be controlled...
// add in conditions for not doing LMR in PV node and then apply this otherwise.

//https://chess.stackexchange.com/questions/15856/implementing-late-move-reduction-lmr-inside-iterative-deepening
//refer this
//has to be tuned down a lot as 5 is very aggressive...
// add in LMR, reducing depth searched
use chess::ChessMove;

pub fn lm_pruning(moves_sorted:Vec<ChessMove>, depth:u8)->Vec<ChessMove>{
    if depth <= 3{
        moves_sorted
    }else if moves_sorted.len()>10{//very aggressive pruning... fine tune.
        moves_sorted[0..5].to_vec()
    }else{
        moves_sorted
    }
}
//add in conditions for checks, etc. this has to be carefully handled or the engine will make erroneous blunders down the line

// fn null_move_pruning{
//
// }
// fn late_move_reduction{
//
// }
// Weiss reduces by 0.20 + ln(depth) * ln(move number) / 3.35 for captures and promotions and 1.35 + ln(depth) * ln(move number) / 2.75 for quiet moves.
// Ethereal reduces by 0.7844 + ln(depth) * ln(moves played) / 2.4696 for quiet moves and 3 (or 2 if the move gave check) for captures and promotions
// Senpai reduces by one ply for the first 6 moves and by depth / 3 for remaining moves.
// Fruit Reloaded uses formula: uint8( sqrt(double(depth-1)) + sqrt(double(moves-1))); for non-PV nodes. In PV-nodes it reduces by 2/3 of this value.