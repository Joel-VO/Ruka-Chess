//add in zobrist hashing to speed up move selection by storing previously reached positions
// will speed up the search by a good margin in theory
use rand::Rng;

const NUM_SQUARES:usize = 64;
const PIECE_NO:usize = 12;//12 pieces
const CASTLING_RIGHTS:usize = 16;// 4*4 possible castles position
const EN_PASSANT:usize = 8;// 8 files for castling
const DEPTH:usize = 0;// depth factor that's checked...used for replacement strategy.

struct ZobristHashing{
    piece_square: [[u64;NUM_SQUARES];PIECE_NO],
    castling_rights: [u64; CASTLING_RIGHTS],
    en_passant_files: [u64; EN_PASSANT],
    depth_factor: u64,
}
impl ZobristHashing {
    fn new_rand()-> Self{
        Self{
            
        }
    }
}