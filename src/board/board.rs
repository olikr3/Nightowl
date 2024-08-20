
/// represents a single Bitboard
#[derive(Debug, Clone, Copy, Hash)]
pub struct BB (u64);

impl BB {

    fn is_square_set(self, square: Square) -> Bool{
       False
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
    nBishop,
    nRook,
    nQueen,
    nKing,
}


///a view of a board, represented by bitboards for individual piece types as well as colors.
pub struct Board {

    pieceBB: [BB; 8],
}

impl Board {
    
    //get piece code for piece
    fn piece_code(&self, pt: Piece_Type) -> usize {
        match pt {
            Piece_Type::nPawn => Piece_Type::nPawn as usize,
            Piece_Type::nBishop => Piece_Type::nBishop as usize,
            Piece_Type::nRook => Piece_Type::nRook as usize,
            Piece_Type::nQueen => Piece_Type::nQueen as usize,
            Piece_Type::nKing => Piece_Type::nKing as usize,
            _ => panic!("Invalid piece type"),
        }
    }

    //get color_codes for piece
    fn color_code(&self, color: Color){
        match color {
            Color::White => Piece_Type::nWhite as usize,
            Color::Black => Piece_Type::nBlack as usize,
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
}

