pub struct MoveGenerator;

impl MoveGenerator {

    /// Generate all legal moves for the given board state.
    pub fn generate_moves(board: &Board, move_list: &mut MoveList) {

        MoveGenerator::generate_pseudo_legal_moves(board, move_list);
        
        MoveGenerator::filter_legal_moves(board, move_list);
    }

    /// Generates pseudo-legal moves (ignoring checks)
    fn generate_pseudo_legal_moves(board: &Board, move_list: &mut MoveList) {

        MoveGenerator::generate_pawn_moves(board, move_list);
        MoveGenerator::generate_knight_moves(board, move_list);
        MoveGenerator::generate_bishop_moves(board, move_list);
        MoveGenerator::generate_rook_moves(board, move_list);
        MoveGenerator::generate_queen_moves(board, move_list);
        MoveGenerator::generate_king_moves(board, move_list);
        MoveGenerator::generate_castling_moves(board, move_list);
    }

    /// Filters out illegal moves that leave the king in check
    fn filter_legal_moves(board: &Board, move_list: &mut MoveList) {

        move_list.moves.retain(|&mv| {
            board.make_move(mv);
            let legal = !board.is_king_in_check(board.side_to_move());
            board.unmake_move(mv);
            legal
        });
    }

    fn generate_pawn_moves(board: &Board, move_list: &mut MoveList) {
    
        todo!
    }

    fn generate_knight_moves(board: &Board, move_list: &mut MoveList) {
        
        todo!
    }

    fn generate_bishop_moves(board: &Board, move_list: &mut MoveList) {
        
        todo!
    }

    fn generate_queen_moves(board: &Board, move_list: &mut MoveList) {

        todo!
    }

    fn generate_king_moves(board: &Board, move_list: &mut MoveList) {
        
        todo!
    }

    fn generate_castling_moves(board: &Board, move_list: &mut MoveList) {

        todo!
    }

}
