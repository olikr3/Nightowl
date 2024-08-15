
// The search engine handles searching for a 'best' move given a list of all possible moves.
// It is implemented as a fail-hard principal variation search.

pub struct Search_Engine {
    moves: Move_List,
    current_position: State,
}

impl Search_Engine {

    fn default(){
        todo!();
    }

    // scans through possible move
    // returns: the best move it found
    fn pvSearch(&self, alpha: u8, beta: u8) -> Move {

        //don't go any deeper - check quiet positions
        if depth == 0 {
            return quiesce(alpha, beta);
        }
        for m in moves.iter(){
            generate_legal_moves(current_position, m); //not yet implemented

        }
    }
}