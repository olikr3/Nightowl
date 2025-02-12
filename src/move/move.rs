
use super::*;

/// represents a single move, following the from-to based approach (see https://www.chessprogramming.org/Encoding_Moves)

/// a Move should never be created manually - otherwise legality cannot be ensured.

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move(u16);

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MoveFlag {
    /// 0000 - Quiet move (normal non-capturing, non-pawn move)
    Quiet = 0b0000,
    
    /// 0001 - Double pawn push (pawn moving 2 squares forward)
    DoublePawnPush = 0b0001,
    
    /// 0010 - King-side castling
    KingCastle = 0b0010,
    
    /// 0011 - Queen-side castling
    QueenCastle = 0b0011,
    
    /// 0100 - Capture (regular piece capture)
    Capture = 0b0100,
    
    /// 0101 - En passant capture
    EnPassantCapture = 0b0101,
    
    /// 1000 - Knight promotion (non-capturing)
    KnightPromotion = 0b1000,
    
    /// 1001 - Bishop promotion (non-capturing)
    BishopPromotion = 0b1001,
    
    /// 1010 - Rook promotion (non-capturing)
    RookPromotion = 0b1010,
    
    /// 1011 - Queen promotion (non-capturing)
    QueenPromotion = 0b1011,
    
    /// 1100 - Knight promotion with capture
    KnightPromoCapture = 0b1100,
    
    /// 1101 - Bishop promotion with capture
    BishopPromoCapture = 0b1101,
    
    /// 1110 - Rook promotion with capture
    RookPromoCapture = 0b1110,
    
    /// 1111 - Queen promotion with capture
    QueenPromoCapture = 0b1111,
}

impl MoveFlag {
    /// Converts a `u8` value to a `MoveFlag`.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0b0000 => Some(MoveFlag::Quiet),
            0b0001 => Some(MoveFlag::DoublePawnPush),
            0b0010 => Some(MoveFlag::KingCastle),
            0b0011 => Some(MoveFlag::QueenCastle),
            0b0100 => Some(MoveFlag::Capture),
            0b0101 => Some(MoveFlag::EnPassantCapture),
            0b1000 => Some(MoveFlag::KnightPromotion),
            0b1001 => Some(MoveFlag::BishopPromotion),
            0b1010 => Some(MoveFlag::RookPromotion),
            0b1011 => Some(MoveFlag::QueenPromotion),
            0b1100 => Some(MoveFlag::KnightPromoCapture),
            0b1101 => Some(MoveFlag::BishopPromoCapture),
            0b1110 => Some(MoveFlag::RookPromoCapture),
            0b1111 => Some(MoveFlag::QueenPromoCapture),
            _ => None, // Invalid flag
        }
    }

    /// Converts `MoveFlag` to its `u8` representation.
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Checks if this move is a capture.
    pub fn is_capture(self) -> bool {
        matches!(
            self,
            MoveFlag::Capture 
            | MoveFlag::EnPassantCapture 
            | MoveFlag::KnightPromoCapture 
            | MoveFlag::BishopPromoCapture 
            | MoveFlag::RookPromoCapture 
            | MoveFlag::QueenPromoCapture
        )
    }

    /// Checks if this move is a promotion.
    pub fn is_promotion(self) -> bool {
        matches!(
            self,
            MoveFlag::KnightPromotion 
            | MoveFlag::BishopPromotion 
            | MoveFlag::RookPromotion 
            | MoveFlag::QueenPromotion 
            | MoveFlag::KnightPromoCapture 
            | MoveFlag::BishopPromoCapture 
            | MoveFlag::RookPromoCapture 
            | MoveFlag::QueenPromoCapture
        )
    }

    /// Checks if this move is a castling move.
    pub fn is_castling(self) -> bool {
        matches!(self, MoveFlag::KingCastle | MoveFlag::QueenCastle)
    }
}

impl Move {
    /// Creates a new move from a given `from`-square, `to`-square, and `flag`
    /// - `from` (6 bits) - Starting square (0-63)
    /// - `to` (6 bits) - Target square (0-63)
    /// - `flag` (4 bits) - Move type flag (e.g., capture, promotion, castle)
    pub fn new(from: u8, to: u8, flag: MoveFlag) -> Self {

        Move(((from as u16) << 10) | ((to as u16) << 4) | (flag.to_u8() as u16))
    }

    /// Extracts the from-square (bits 10-15)
    pub fn from(self) -> u8 {
        (self.0 >> 10) as u8
    }

    /// Extracts the to-square (bits 4-9)
    pub fn to(self) -> u8 {
        ((self.0 >> 4) & 0b111111) as u8
    }

    /// Extracts the move flag (bits 0-3)
    pub fn flag(self) -> Option<MoveFlag> {
        MoveFlag::from_u8((self.0 & 0b1111) as u8)
    }

    /// Checks if this move is a capture.
    pub fn is_capture(self) -> bool {
        self.flag().map_or(false, |flag| flag.is_capture())
    }

    /// Checks if this move is a promotion.
    pub fn is_promotion(self) -> bool {
        self.flag().map_or(false, |flag| flag.is_promotion())
    }

    /// Checks if this move is a castling move.
    pub fn is_castling(self) -> bool {
        self.flag().map_or(false, |flag| flag.is_castling())
    }
}