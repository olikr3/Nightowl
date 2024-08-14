/*
use Nightowl::board::board::BB;
use Nightowl::board::get_white_pawns;
use Nightowl::board::get_black_pawns;
use Nightowl::board::Piece_Type;

#[test]
    fn test_get_white_pawns() {
        let mut board = BB {
            piece_bb: [0; 8],
        };

        // Set white pawns on the second rank (0x000000000000FF00)
        board.piece_bb[Piece::Pawn as usize] = 0x000000000000FF00;
        board.piece_bb[Piece::White as usize] = 0x000000000000FFFF;

        assert_eq!(board.get_white_pawns(), 0x000000000000FF00);
    }
*/