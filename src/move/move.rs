

/// represents a single move along with the following flags encoded in bits
/// 1    capture
/// 2    castle
/// 4    en passant capture
/// 8    pushing a pawn 2 squares
/// 16    pawn move
/// 32    promote
/// 
/// 
/// courtesy of Joey Robert, see: https://joeyrobert.org/2009/03/08/layout-of-a-chess-engine/
/// 

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move {
    from: u64,
    to: u64,
    flags: u64,
}

/// not yet decided whether to use this enum or the flags variable
/*
pub enum MoveFlag {
    Promotion{
        capture_promotion: bool,
        promote_to: Piece_Type,
    },
    Castle{
        king_side_castle: bool,
    },
    Capture{
        en_passant: bool,
    },
    DoublePawnPush,
    Standard, // a standard move
}
*/

impl Move {

    fn new(from: u64, to: u64, flags: u64) -> Self {
        Move {
            from,
            to,
            flags,
        }
    }

    fn apply_move(board: &mut Board, mv: Move) -> &Board {
        if is_legal(mv) {
            board &= mv;
        }
        return &Board
    }

    fn is_legal(board: &mut Board, mv: Move) -> Bool {

    }
}