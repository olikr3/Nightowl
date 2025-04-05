pub struct MoveGenerator;

impl MoveGenerator {

    /// Generate all legal moves for the given board state.
    /// by generating all possible moves and then filtering out illegal ones

    pub fn generate_moves(board: &Board, move_list: &mut MoveList) {

        MoveGenerator::generate_pseudo_legal_moves(board, move_list);
        
        MoveGenerator::filter_legal_moves(board, move_list);
    }

    fn generate_pseudo_legal_moves(board: &Board, move_list: &mut MoveList) {

        MoveGenerator::generate_pawn_moves(board, move_list);
        MoveGenerator::generate_knight_moves(board, move_list);
        MoveGenerator::generate_bishop_moves(board, move_list);
        MoveGenerator::generate_rook_moves(board, move_list);
        MoveGenerator::generate_queen_moves(board, move_list);
        MoveGenerator::generate_king_moves(board, move_list);
        MoveGenerator::generate_castling_moves(board, move_list);
    }

    fn filter_legal_moves(board: &Board, move_list: &mut MoveList) {

        move_list.moves.retain(|&mv| {
            board.make_move(mv); // not yet implemented
            let legal = !board.is_king_in_check(board.side_to_move());
            board.unmake_move(mv); // not yet implemented
            legal
        });
    }

    fn generate_pawn_moves(board: &Board, move_list: &mut MoveList) {
    
        todo()!
    }

    fn generate_knight_moves(board: &Board, move_list: &mut MoveList) {
        
        todo()!
    }

    fn generate_bishop_moves(board: &Board, move_list: &mut MoveList) {
        
        todo()!
    }

    fn generate_queen_moves(board: &Board, move_list: &mut MoveList) {

        todo()!
    }

    fn generate_king_moves(board: &Board, move_list: &mut MoveList) {
        
        todo()!
    }

    fn generate_castling_moves(board: &Board, move_list: &mut MoveList) {

        todo()!
    }

}
