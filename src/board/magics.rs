
/// Fancy magics as proposed by Pradu Kannan (see https://www.chessprogramming.org/Magic_Bitboards)

#[derive(Debug, Clone, Copy)]
pub struct Magic {
    magic: u64,         // The magic number
    mask: BB,           // Relevant occupancy mask
    shift: u8,          // Shift to reduce bits
    attack_table: Vec<BB>, // Precomputed attack bitboards
}

impl Magic {
    fn new(magic: u64, mask: BB, shift: u8, attack_table: Vec<BB>) -> Self {
        Self {
            magic,
            mask,
            shift,
            attack_table,
        }
    }
}