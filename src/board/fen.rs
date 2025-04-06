

/// this struct is responsible for creating boards from FEN-Strings.
pub struct FenParser {}

impl FenParser {

    /// creates a Board from a valid FEN-String
    fn from_string(fen: &str) -> Result<Board, &'static str> {
        
        if !isvalid(fen) {
            return Err("Invalid fen");
        }
        b: Board = Board {
            todo!()
        }
        Ok(b)
    }


    /// this functions assumes that the input fen string is valid
    /// it might make more senst to write an iterator to process the string
    fn build_board(fen: &'static str) -> Board {

        // naming differences to board struct are atrocious for now
        let parts = fen.split_whitespace().collect();
        let pieces = parts[0].to_string(); // need to create the bbs for them
        let active_color = parts[1].to_string();
        let castling = parts[2].to_string();
        let en_passant_target_square = parts[3].to_string();
        let halfmove_clock = parts[4].to_string();
        let fullmove_number = parts[5].to_string();

        // todo: match all strs to correct format
        // todo: write separate constructor in board
        Board {
            pieceBB: build_bbs(pieces),
            side_to_move, // todo: match str to color
            castling_rights: castling,
            en_passant_square: en_passant_target_square,
            half_moves: halfmove_clock as u8,
            full_moves: fullmove_number as u8,

        }
    }

    /// build all required biboards for the board struct
    /// a bit tricky
    fn build_bbs(&'static str pcs) -> [pieceBB] {
        
       /// sth like:
       /// build_pawn_bb()
       /// build_knight_bb()
       /// etc.
        todo!();
    }

    fn build_pawn_bb(&'static str pcs) -> u64 {}

    fn build_knight_bb(&'static str pcs) -> u64 {}

    fn build_bishop_bb(&'static str pcs) -> u64 {}

    fn build_rook_bb(&'static str pcs) -> u64 {}

    fn build_queen_bb(&'static str pcs) -> u64 {}

    fn build_king_bb(&'static str pcs) -> u64 {}


    fn to_color(&'static str color) -> Color {
       
       match color {
            "w" => return Color::White,
            _ => return Color::Black,
       }
    }

    fn is_valid(&'static str fen) -> bool {
    }

}

