

use bitvec::prelude::*;

use crate::append_to_bitvec;


fn add_terminator(bitvec: &mut BitVec) {
    let len_terminator: usize = check_terminator_len(&bitvec);

    let mut terminator_bitvec: BitVec = bitvec![0; len_terminator];
    bitvec.append(&mut terminator_bitvec);
}

fn check_terminator_len(bitvec: &BitVec) -> usize {
    let mut len_terminator = 0;
    if bitvec.len() < 152 {
        len_terminator = 152 - bitvec.len();

        if len_terminator >= 4 {
            len_terminator = 4;
        }
    }

    return len_terminator;
}

fn add_more_zero_to_multiply_8(bitvec: &mut BitVec) {
    let bitvec_zero_len: usize = bitvec.len() % 8;

    let mut zero_bitvec: BitVec = bitvec![0; bitvec_zero_len];

    bitvec.append(&mut zero_bitvec);
}

fn add_pad_bytes(bitvec: &mut BitVec) {
    for pad in [11101100, 00010001].iter().cycle() {
        if bitvec.len() >= 152 {
            break;
        }

        append_to_bitvec(bitvec, &pad, 8);
    }
}

pub fn add_bits_to_required_len(bitvec: &mut BitVec) {
    add_terminator(bitvec);

    add_more_zero_to_multiply_8(bitvec);

    add_pad_bytes(bitvec);
} 