use bitvec::prelude::*;

use crate::append_to_bitvec;
use crate::data_mode::Mode;

pub fn get_bitvector_char_counter(character: &String, mode: &Mode) -> BitVec {
    let mut bitvec: BitVec = BitVec::new();

    let character_len: u32 = character.len().try_into().unwrap();
    
    let bit_len;

    match mode {
        Mode::Numeric => bit_len = 10,
        Mode::Byte => bit_len = 8,
        _ => bit_len = 10,
    }

    append_to_bitvec(&mut bitvec, &character_len, bit_len);

    return bitvec;
}
