
/// Black magics as proposed by Volker Annuss (see https://www.chessprogramming.org/Magic_Bitboards)

const ATTACK_TABLE_SIZE = 88507;

static mut ATTACK_TABLE: [BB; ATTACK_TABLE_SIZE]

static mut BISHOP_MAGIC: [BlackMagic; 64] = [BlackMagic {
    ptr: &[],
    notmask: 0,
    black_magic: 0,
}; 64];

static mut ROOK_MAGIC: [BlackMagic; 64] = [BlackMagic {
    ptr: &[],
    notmask: 0,
    black_magic: 0,
}; 64];

#[derive(Debug, Clone, Copy)]
pub struct BlackMagic {
    ptr: &'static [BB], // reference to attack table slice
    notmask: BB; // for relevant occupancy bits
    black_magic: BB,
}

impl BlackMagic {}

fn get_rook_attacks(occ: BB, square: usize) -> BB {}
fn get_bishop_attacks(occ: BB, square: usize) -> BB {}
