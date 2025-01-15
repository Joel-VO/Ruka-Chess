//add in zobrist hashing to speed up move selection by storing previously reached positions
// will speed up the search by a good margin in theory

use chess::{Board, Piece, Square};
use rand::{thread_rng, Rng};

const NUM_SQUARES:usize = 64;
const PIECE_NO:usize = 12;//12 pieces
const CASTLING_RIGHTS:usize = 16;// 4*4 possible castles position
const EN_PASSANT:usize = 8;// 8 files for castling
const DEPTH:usize = 0;// depth factor that's checked...used for replacement strategy.

pub struct ZobristHashing{
    pub piece_square: [[u64;NUM_SQUARES];PIECE_NO],
    pub castling_rights: [u64; CASTLING_RIGHTS],
    pub en_passant_files: [u64; EN_PASSANT],
    pub depth_factor: u64,
}
impl ZobristHashing { //generates a random hash number every time its called and this is used as a
    // key to compute the respective hash value. Use of 64 bits means collisions are 1 in a 100 million or so
    pub fn new_table()-> Self{
        let mut rng = thread_rng();
        Self{
            piece_square:[[0;NUM_SQUARES];PIECE_NO].map(|row| row.map(|_| rng.gen())),
            castling_rights: [0; CASTLING_RIGHTS].map(|_| rng.gen()),
            en_passant_files: [0; EN_PASSANT].map(|_| rng.gen()),
            depth_factor: 0,
        }
    }
}

fn compute_hash_value(board:&Board, zobrist_key:&ZobristHashing) -> u64{
    let mut hash:u64 = 0;

    // add logic for each piece

    for square in 0..NUM_SQUARES{
        let position = unsafe {Square::new(square as u8)};
        //could have used file and rank function, but might have been slower due to extra computation
        // this is unsafe but only because of values greater than 64, which won't be violated here
        if let Some(piece) = board.piece_on(position){// to check if such a piece exists
            let piece_val:usize = piece.to_index();
            hash^=zobrist_key.piece_square[square][piece_val];
        }
    }

    //add logic for castling
    //add logic for en passant


    hash
}