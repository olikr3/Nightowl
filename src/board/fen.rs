

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
    fn build_board(fen: &str) -> Board {

        let parts = fen.split_whitespace().collect();
        let pieces = parts[0].to_string(); // need to create the bbs for them
        let active_color = parts[1].to_string();
        let castling = todo!();
        let en_passant = todo!();
        let halfmove_clock = todo!();
        let en_passant_target_square = todo()!;

        Board {} //todo
    }

}
