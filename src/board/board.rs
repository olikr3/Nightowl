
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
    nWhite,
    nBlack,
    nPawn,
    nKnight,
    nBishop,
    nRook,
    nQueen,
    nKing,
}


///a view of a board, represented by bitboards for individual piece types as well as colors.
pub struct Board {

    pieceBB: [BB; 8],
    side_to_move: Color,
    castling: Castling,
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
            _ => panic!("Invalid piece type"),
        }
    }

    //return the color_code mapped to a specific piece
    fn color_code(&self, color: Color){
        match color {
            Color::White => Piece_Type::nWhite as u64,
            Color::Black => Piece_Type::nBlack as u64,
            _ => panic!("Invalid color"),
        }
    }

    // return a specific piece set, i.e. an intersection between a color and a piece type
    fn get_piece_set(&self, pt: PieceType, cC: colorCode) -> BB {
        self.pieceBB[Self::piece_code(pt)] & self.pieceBB[Self::color_code(cC)]
    }

    fn get_white_pawns(&self) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.pieceBB[Self::color_code(Color::White)]
    }

    fn get_black_pawns(&self) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.pieceBB[Self::color_code(Color::Black)]
    }

    fn get_pawns(&self, ct: Color) -> BB {
        self.pieceBB[Self::piece_code(Piece_Type::nPawn)] & self.piece_bb[Self::color_code]
    }

    fn get_bitboard(pt: Piece_Type) -> BB {
        self.piece_bb[Self::piece_code(pt)]
    }
}

