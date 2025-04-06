

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


}
