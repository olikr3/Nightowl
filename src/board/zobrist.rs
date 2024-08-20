

/// The idea of zobrist hashing is to generate a unique hash for each chess position.
/// This hash is then used to determine whether a specific position has been encountered before.
/// This is done by using XOR - operations on pieces, castling rights, etc. to create a unique identifier.

use rand::Rng;

const PIECE_TYPES: usize = 12; // 6 for white, 6 for black
const SQUARES: usize = 64;

struct Zobrist {
    piece_square_keys: [[u64; SQUARES]; PIECE_TYPES],
    side_to_move_key: u64,
    castling_rights_keys: [u64; 16], // 4 castling rights (KQkq)
    en_passant_keys: [u64; 8], // 8 possible files for en passant
}

impl Zobrist {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut piece_square_keys = [[0u64; SQUARES]; PIECE_TYPES];
        let mut side_to_move_key = rng.gen::<u64>();
        let mut castling_rights_keys = [0u64; 16];
        let mut en_passant_keys = [0u64; 8];

        for piece in 0..PIECE_TYPES {
            for square in 0..SQUARES {
                piece_square_keys[piece][square] = rng.gen::<u64>();
            }
        }

        for i in 0..16 {
            castling_rights_keys[i] = rng.gen::<u64>();
        }

        for i in 0..8 {
            en_passant_keys[i] = rng.gen::<u64>();
        }

        Zobrist {
            piece_square_keys,
            side_to_move_key,
            castling_rights_keys,
            en_passant_keys,
        }
    }
    
}
