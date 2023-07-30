

use bitvec::prelude::*;

use crate::append_to_bitvec;

pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

impl Mode {
    pub fn get_bitvec(mode: &Mode) -> BitVec {
        let mut bitvec: BitVec = BitVec::new();
        match mode {
            Mode::Numeric => append_to_bitvec(&mut bitvec, &1, 4),
            //Mode::Alphanumeric => return BitVec::from_vec(vec![0, 0, 1, 0]),
            //Mode::Byte => BitVec::from_vec(vec![0, 1, 0, 0]),
            _ => return bitvec,
        }
        return bitvec;
    }
}