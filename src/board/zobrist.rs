

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
        let mut piece_square_keys = [[0u64; SQUARES]; PIECE_TYPES]; // 12 arrays of length 64 filled with 64 bit random numbers
        let mut side_to_move_key = rng.gen::<u64>();
        let mut castling_rights_keys = [0u64; 16]; // there are 2^4 different combinations for castling rights
        let mut en_passant_keys = [0u64; 8]; // a-h files  representing a possible en passant file

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

impl Zobrist {
    fn hash_position(&self, board: &Board) -> u64 {
        let mut hash = 0u64;

        // XOR the piece-square keys
        for piece in 0..PIECE_TYPES {
            let mut piece_bb = board.get_bitboard(piece);
            while piece_bb != 0 {
                let square = piece_bb.trailing_zeros() as usize;
                hash ^= self.piece_square_keys[piece][square];
                piece_bb &= piece_bb - 1;
            }
        }

        // XOR the side to move key
        if bitboard.side_to_move == Color::Black {
            hash ^= self.side_to_move_key;
        }

        // XOR the castling rights keys
        hash ^= self.castling_rights_keys[board.castling_rights as usize];

        // XOR the en passant key if there's an en passant square
        if let Some(ep_square) = board.en_passant_square {
            let file = ep_square % 8;
            hash ^= self.en_passant_keys[file as usize];
        }

        hash
    }
}

impl Zobrist {
    fn update_hash(
        &self,
        hash: u64,
        piece: usize,
        from_square: usize,
        to_square: usize,
        side_to_move: Color,
        old_castling_rights: u8,
        new_castling_rights: u8,
        old_ep_square: Option<usize>,
        new_ep_square: Option<usize>,
    ) -> u64 {
        let mut new_hash = hash;

        // Remove the piece from the old square
        new_hash ^= self.piece_square_keys[piece][from_square];

        // Place the piece on the new square
        new_hash ^= self.piece_square_keys[piece][to_square];

        // Update the side to move
        new_hash ^= self.side_to_move_key;

        // Update castling rights
        if old_castling_rights != new_castling_rights {
            new_hash ^= self.castling_rights_keys[old_castling_rights as usize];
            new_hash ^= self.castling_rights_keys[new_castling_rights as usize];
        }

        // Update en passant square
        if old_ep_square != new_ep_square {
            if let Some(old_ep) = old_ep_square {
                new_hash ^= self.en_passant_keys[old_ep % 8];
            }
            if let Some(new_ep) = new_ep_square {
                new_hash ^= self.en_passant_keys[new_ep % 8];
            }
        }

        new_hash
    }
}

