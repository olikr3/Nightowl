

/// for now mostly just pseudocode, as some details are still unclear
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
        Board {
            pieceBB, //todo: write method for that
            side_to_move: active_color, // todo: match str to color
            castling_rights: castling,
            en_passant_square: en_passant_target_square,
            // why do i need occupied and empty in board -> todo: go over that
            half_moves: halfmove_clock,
            full_moves: fullmove_number,

        } //todo
    }

    // question to self: should this even be in parser?
    /// build all required biboards for the board struct
    fn build_bbs(&'static str pieces) -> [pieceBB] {
        todo!();
    }

    fn to_color(&'static str pieces) -> Color {
        
        todo!();
    }

}

