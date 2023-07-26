

use bitvec::prelude::*;

use crate::append_to_bitvec;


pub fn add_terminator(data_bitvec: &mut BitVec, bitvec_len: &mut usize) {
    let len_terminator: usize = check_terminator_len(*bitvec_len);

    let mut terminator_bitvec: BitVec = bitvec![0; len_terminator];
    data_bitvec.append(&mut terminator_bitvec);

    *bitvec_len += len_terminator;
}

fn check_terminator_len(bitvec_len: usize) -> usize {
    let mut len_terminator = 0;
    if bitvec_len < 152 {
        len_terminator = 152 - bitvec_len;

        if len_terminator >= 4 {
            len_terminator = 4;
        }
    }

    return len_terminator;
}

pub fn add_more_zero_to_multiply_8(data_bitvec: &mut BitVec, bitvec_len: &mut usize) {
    let bitvec_zero_len: usize = *bitvec_len % 8;

    let mut zero_bitvec: BitVec = bitvec![0; bitvec_zero_len];

    data_bitvec.append(&mut zero_bitvec);

    *bitvec_len += bitvec_zero_len;
}

fn add_pad_bytes(data_bitvec: &mut BitVec, bitvec_len: &mut usize) {
    for pad in [0b11101100, 0b00010001].iter().cycle() {
        if *bitvec_len >= 152 {
            break;
        }

        append_to_bitvec(data_bitvec, &pad, 8);
        *bitvec_len += 8;
    }
}

pub fn add_bits_to_required_len(data_bitvec: &mut BitVec, mut bitvec_len: usize) {
    add_terminator(data_bitvec, &mut bitvec_len);

    add_more_zero_to_multiply_8(data_bitvec, &mut bitvec_len);

    add_pad_bytes(data_bitvec, &mut bitvec_len);
} 