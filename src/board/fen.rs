pub struct FenParser {}

impl FenParser {
    pub fn from_string(fen: &str) -> Result<Board, &'static str> {
        if !FenParser::is_valid(fen) {
            return Err("Invalid FEN");
        }
        Ok(FenParser::build_board(fen))
    }

    fn build_board(fen: &str) -> Board {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let pieces = parts[0];
        let active_color = parts[1];
        let castling = parts[2];
        let en_passant = parts[3];
        let halfmove_clock = parts[4].parse::<u8>().unwrap_or(0);
        let fullmove_number = parts[5].parse::<u8>().unwrap_or(1);

        Board {
            pieceBB: FenParser::build_bbs(pieces),
            side_to_move: FenParser::to_color(active_color),
            castling_rights: FenParser::parse_castling(castling),
            en_passant_square: FenParser::parse_en_passant(en_passant),
            half_moves: halfmove_clock,
            full_moves: fullmove_number,
        }
    }

    fn build_bbs(pcs: &str) -> [BB; 8] {
        let mut pieceBB = [BB(0); 8];
        let mut rank = 7;
        let mut file = 0;

        for c in pcs.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
            } else if c.is_digit(10) {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let index = FenParser::piece_to_index(c);
                pieceBB[index].0 |= 1 << (rank * 8 + file);
                file += 1;
            }
        }
        pieceBB
    }

    fn piece_to_index(piece: char) -> usize {
        match piece {
            'P' => 0,
            'N' => 1,
            'B' => 2,
            'R' => 3,
            'Q' => 4,
            'K' => 5,
            'p' => 6,
            'n' => 7,
            'b' => 8,
            'r' => 9,
            'q' => 10,
            'k' => 11,
            _ => panic!("Invalid piece character"),
        }
    }

    fn to_color(color: &str) -> Color {
        match color {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid color"),
        }
    }

    fn parse_castling(castling: &str) -> u8 {
        let mut rights = 0;
        if castling.contains('K') { rights |= 1; }
        if castling.contains('Q') { rights |= 2; }
        if castling.contains('k') { rights |= 4; }
        if castling.contains('q') { rights |= 8; }
        rights
    }

    fn parse_en_passant(en_passant: &str) -> Option<u8> {
        if en_passant == "-" {
            None
        } else {
            let file = en_passant.chars().nth(0).unwrap() as u8 - b'a';
            let rank = en_passant.chars().nth(1).unwrap().to_digit(10).unwrap() as u8 - 1;
            Some(rank * 8 + file)
        }
    }

    fn is_valid(fen: &str) -> bool {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        parts.len() == 6
    }

    fn to_fen(board: &Board) -> String {
        todo!()
    }
}


