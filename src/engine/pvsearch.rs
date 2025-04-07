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
    flag: ScopeFlag,
}

#[derive(Clone, Copy, PartialEq)]
enum ScopeFlag {
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

    pub fn search(&mut self, board: &mut Board, max_depth: i32) -> Option<Move> {
        let mut best_move = None;
        let mut alpha = -CHECKMATE;
        let beta = CHECKMATE;

        for depth in 1..=max_depth {
            let mut move_list = MoveList::new();
            MoveGenerator::generate_moves(board, &mut move_list);
            self.order_moves(&mut move_list, board); // not implemented

            let mut best_score = -CHECKMATE;
            for mv in move_list.moves.iter() {
                board.make_move(*mv);
                let score = -self.pv_search(board, -beta, -alpha, depth - 1);
                board.unmake_move(*mv);

                if score > best_score {
                    best_score = score;
                    best_move = Some(*mv);
                }

                if score > alpha {
                    alpha = score;
                }
            }
        }

        best_move
    } 
}
