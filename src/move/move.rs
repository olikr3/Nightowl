

pub struct Move {
    from: usize,
    to: usize,
    flags: usize,
}

impl Move {

    fn new(from: usize, to: usize, flags: Option<usize>) -> Self {
        Move {
            from,
            to,
            flags,
        }
    }
}