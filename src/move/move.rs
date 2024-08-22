

pub struct Move {
    from: u64,
    to: u64,
    flags: u64,
}

impl Move {

    fn new(from: u64, to: u64, flags: Option<u64>) -> Self {
        Move {
            from,
            to,
            flags,
        }
    }

    fn make_move(board: &mut Board, mv: Move) -> None {
        todo!()
    }
}