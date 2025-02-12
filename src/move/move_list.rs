#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveList {
    moves: [Move; 256], // Max possible legal moves should be lower than 256
    count: usize,
}

impl MoveList {
    /// Creates empty MoveList
    pub fn new() -> Self {
        MoveList {
            moves: [Move(0); 256],
            count: 0,
        }
    }

    /// Adds a move to the list
    pub fn add(&mut self, mv: Move) {
        if self.count < 256 {
            self.moves[self.count] = mv;
            self.count += 1;
        }
    }

    /// Returns the number of moves in the list
    pub fn len(&self) -> usize {
        self.count
    }

    /// Checks if the list is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Get move by index
    pub fn get(&self, index: usize) -> Option<Move> {
        if index < self.count {
            Some(self.moves[index])
        } else {
            None
        }
    }

    /// Clears the move list
    pub fn clear(&mut self) {
        self.count = 0;
    }

    /// Iterates over the stored moves
    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.moves[..self.count].iter()
    }
}