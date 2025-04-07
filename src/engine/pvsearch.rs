use std::collections::HashMap;

const CHECKMATE: i32 = 10_000;
const TRANSPOSITION_TABLE_SIZE: usize = 1_000_000;

/// handles the principle variation search
pub struct PVSearchHandler {
    transposition_table: HashMap<u64, TranspositionEntry>,
}

#[derive(Clone, Copy)]
struct TranspositionEntry {
    zobrist_key: u64,
    depth: i32,
    value: i32,
    flag: HashFlag,
}

#[derive(Clone, Copy, PartialEq)]
enum Flag {
    Exact,
    Alpha,
    Beta,
}

impl PVSearch {

    pub fn new() -> Self {
        PVSearchHandler {
            transposition_table:
            HashMap::with_capacity(TRANSPOSITION_TABLE_SIZE),
        }
    }

    pub fn search(&mut self, board:&mut Board, depth: i32) -> Option<Move> {
        todo!()
    } 
}
