
use bitvec::prelude::*;

use crate::append_to_bitvec;

pub fn get_bitvector_char_counter(character: &String) -> BitVec {
    let mut bitvec: BitVec = BitVec::new();
    bitvec.reserve(10);

    let character_len: u32 = character.len().try_into().unwrap();
    append_to_bitvec(&mut bitvec, &character_len, 10);
        
    return bitvec;
}