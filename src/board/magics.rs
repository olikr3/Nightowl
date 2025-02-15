
/// Black magics as proposed by Volker Annuss (see https://www.chessprogramming.org/Magic_Bitboards)

const ATTACK_TABLE_SIZE = 88507;

static mut ATTACK_TABLE: [BB; ATTACK_TABLE_SIZE]


#[derive(Debug, Clone, Copy)]
pub struct BlackMagic {
    ptr: &'static [BB], // reference to attack table slice
    notmask: BB; // for relevant occupancy bits
    black_magic: bb,
}

