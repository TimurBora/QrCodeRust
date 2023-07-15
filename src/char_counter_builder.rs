
use bitvec::prelude::*;

use crate::append_to_bitvec;

fn get_bitvector_char_counter(bitvector: &mut BitVec, character: String) {
    let mut bitvec: BitVec = BitVec::new();
    bitvec.reserve(10);

    let character_len: u32 =  character.len();
    append_to_bitvec(&bitvec, character_len, 10);
        
    return bitvec;
}