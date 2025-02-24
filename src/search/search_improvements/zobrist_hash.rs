// //add in zobrist hashing to speed up move selection by storing previously reached positions
// // will speed up the search by a good margin in theory
//
// use chess::{Board, ChessMove, Square,Color};
// use rand::{thread_rng, Rng};
//
// const NUM_SQUARES:usize = 64;
// const PIECE_NO:usize = 12;//12 pieces
// const CASTLING_RIGHTS:usize = 16;// 4*4 possible castles position
// const EN_PASSANT:usize = 8;// 8 files for castling
//
// pub struct ZobristHashing{
//     pub piece_square: [[u64;NUM_SQUARES];PIECE_NO],
//     pub castling_rights: [u64; CASTLING_RIGHTS],
//     pub en_passant_files: [u64; EN_PASSANT],
//     pub side_to_move:u64
// }
// impl ZobristHashing { //generates a random hash number every time its called and this is used as a
//     // key to compute the respective hash value. Use of 64 bits means collisions are 1 in a 100 million or so
//     pub fn new_table()-> Self{
//         let mut rng = thread_rng();
//         Self{
//             piece_square:[[0;NUM_SQUARES];PIECE_NO].map(|row| row.map(|_| rng.gen())),
//             castling_rights: [0; CASTLING_RIGHTS].map(|_| rng.gen()),
//             en_passant_files: [0; EN_PASSANT].map(|_| rng.gen()),
//             side_to_move: rng.gen()
//         }
//     }
// }
// //A special mention to my girlfriend... who has supported me in making this bot, has put up with my obsession in creating Ruka and has been my pillar in its creation.
// pub fn compute_hash_value(board:&Board, zobrist_key:&ZobristHashing) -> u64{
//     let mut hash:u64 = 0;
//
//     // add logic for each piece
//
//     for square in 0..NUM_SQUARES{
//         let position:Square = unsafe {Square::new(square as u8)};
//         //could have used file and rank function, but might have been slower due to extra computation
//         // this is unsafe but only because of values greater than 64, which won't be violated here
//         if let Some(piece) = board.piece_on(position){// to check if such a piece exists
//             let piece_val:usize = piece.to_index();
//             hash^=zobrist_key.piece_square[piece_val][square];//XOR hash with the randomly generated value
//         }
//     }
//     //add logic for castling
//     // for (i, right) in board.castle_rights().iter().enumerate() {
//     //     if *right {
//     //         hash ^= zobrist_key.castling_rights[i];
//     //     }
//     // }
//     //add logic for en passant
//     if board.side_to_move() == Color::Black{
//         hash ^= zobrist_key.side_to_move
//     }
//     hash
// }
//
// pub fn updated_hash_move(current_hash:u64, move_made:&ChessMove, zobrist_key:&ZobristHashing, board:&Board)->u64{
//     //When passing the board, make sure the move is not made in the board!!! otherwise the kernel will panic
//     let mut new_hash = current_hash;
//     let index_piece_start = move_made.get_source().to_int() as usize;
//     let index_piece_end = move_made.get_dest().to_int() as usize;
//     if let Some(piece) = board.piece_on(move_made.get_source()) {
//         let piece_index = piece.to_index();
//         new_hash ^= zobrist_key.piece_square[piece_index][index_piece_start];
//         new_hash ^= zobrist_key.piece_square[piece_index][index_piece_end];
//     } else {
//         panic!("Source square is empty! Invalid move.");
//     }
//     new_hash ^= zobrist_key.side_to_move;
//     new_hash
// }