
/// Represents a single Bitboard
#[derive(Debug, Clone, Copy, Hash)]
pub struct BB(u64);

impl BB {
    // check if a specific square is set
    fn is_square_set(&self, square: Square) -> bool {
        self.0 & (1 << square) != 0
    }

    // check if bitboard is empty
    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    // check if bitboard is singly populated (i.e. exactly one bit is set)
    fn is_singly_populated(&self) -> bool {
        self.0 != 0 && (self.0 & (self.0 - 1)) == 0
    }

    // count the number of set bits (Hamming weight)
    fn pop_count(&self) -> usize {
        self.0.count_ones() as usize
    }
}


#[derive(Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
enum Piece_Type {
    nWhite, // any white piece
    nBlack, // any black piece
    nPawn,
    nKnight,
    nBishop,
    nRook,
    nQueen,
    nKing,
}

/// the Board struct contains basically all the information that is parsed from
/// a fen string
pub struct Board {

    pieceBB: [BB; 8], // bitboards for individual piece types as well as colors.
    side_to_move: Color, // Color to move
    castling_rights: u8, // Castling rights flags
    en_passant_square: Option<u8>, // Square where en passant is possible
    half_moves: u8,
    full_moves: u8,

}

impl Board {
    
    //return the piece_code mapped to a specific piece
    fn piece_code(&self, pt: Piece_Type) -> u64 {
        match pt {
            Piece_Type::nPawn => Piece_Type::nPawn as u64,
            Piece_Type::nBishop => Piece_Type::nBishop as u64,
            Piece_Type::nKnight => Piece_Type::nKnight as u64,
            Piece_Type::nRook => Piece_Type::nRook as u64,
            Piece_Type::nQueen => Piece_Type::nQueen as u64,
            Piece_Type::nKing => Piece_Type::nKing as u64,
        }
    }

    //return the color_code mapped to a specific piece
    fn color_code(&self, color: Color){
        match color {
            Color::White => Piece_Type::nWhite as u64,
            Color::Black => Piece_Type::nBlack as u64,
        }
    }

    // return a specific piece set, i.e. an intersection between a color and a piece type
    fn get_piece_set(&self, pt: Piece_Type, cC: Color) -> BB {
        self.pieceBB[Self::piece_code(pt)] & self.pieceBB[Self::color_code(cC)]
    }

    fn get_white_pawns(&self) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.pieceBB[Self::color_code(Color::White)]
    }

    fn get_black_pawns(&self) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.pieceBB[Self::color_code(Color::Black)]
    }

    fn get_pawns(&self, ct: Color) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.pieceBB[Self::color_code]
    }

    fn get_bitboard(&self, pt: Piece_Type) -> BB {
        self.pieceBB[Self::piece_code(pt)]
    }

    fn from_fen(fen: &str) -> Self {
        fp = FenParser::from_string(fen);
        b = fp::build_board(fen);
        B
    }

    fn make_move(mv: Move) -> Self {
        
        let from = mv.from();
        let to = mv.to();
        let flags = mv.flag();

        let mut moved_piece = None;
        for (i, bb) in self.pieceBB.iter_mut.enumerate() {
            if bb.is_square_set(from) {
                moved_piece = Some(i);
                bb.0 &= (1 << from);
                break;
            }
        }
        if let Some(piece_idx) = moving_piece {
            self.pieceBB[piece_idx].0 |= 1 << to;
        }

        // Handle captures
        if let Some(flag) = flag {
            if flag.is_capture() {
                for bb in self.pieceBB.iter_mut() {
                    if bb.is_square_set(to as u8) {
                        bb.0 &= !(1 << to);
                        break;
                    }
                }
            }

            match flag {
                MoveFlag::DoublePawnPush => {
                    self.en_passant_square = Some(to as u8);
                }
                MoveFlag::EnPassantCapture => {
                    let ep_capture_square = if self.side_to_move == Color::White { to - 8 } else { to + 8 };
                    self.pieceBB[self.piece_code(Piece_Type::nPawn) as usize].0 &= !(1 << ep_capture_square);
                }
                MoveFlag::KingCastle => {
                    self.pieceBB[self.piece_code(Piece_Type::nRook) as usize].0 &= !(1 << (to + 1)); // Remove rook from old square
                    self.pieceBB[self.piece_code(Piece_Type::nRook) as usize].0 |= 1 << (to - 1); // Move rook
                }
                MoveFlag::QueenCastle => {
                    self.pieceBB[self.piece_code(Piece_Type::nRook) as usize].0 &= !(1 << (to - 2));
                    self.pieceBB[self.piece_code(Piece_Type::nRook) as usize].0 |= 1 << (to + 1);
                }
                _ => {}
            }
        }

        // Switch turn
        self.side_to_move = match self.side_to_move {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        // Update move counters
        self.half_moves += 1;
        if self.side_to_move == Color::White {
            self.full_moves += 1;
        }
    }


    fn unmake_move(mv: Move) -> Self {

        // i am not sure how best to implement this, probabaly i need a structure to preserve the history of moves first
        todo!()
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        let king_pos = self.get_king_position(color);
        let opponent_color = match color {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        
        let mut opponent_moves = MoveList::new();
        MoveGenerator::generate_pseudo_legal_moves(self, &mut opponent_moves);
        
        for mv in opponent_moves.moves {
            if mv.to() == king_pos {
                return true;
            }
        }
        false
    }

    fn get_king_position(&self, color: Color) -> u8 {
        let king_bb = self.pieceBB[self.piece_code(Piece_Type::nKing) as usize];
        for i in 0..64 {
            if king_bb.is_square_set(i) {
                return i;
            }
        }
        panic!("_");
    }
}

