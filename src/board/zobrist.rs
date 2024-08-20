
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
    fn new()
}
