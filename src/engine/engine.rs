
pub struct Engine {
    board: Board,
    search: PVSearchHandler,
}

impl Engine {

    /// creates engine from starting position
    pub fn default() -> Self {
        todo!()
    }
    
    /// initialises engine with custom position
    pub fn from_position(&'static str fen) -> Self {
        fp = FenParser::new(fen);
        board = fp.from_string(fen);
        todo!()
    }
}
