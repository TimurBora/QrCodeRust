

use bitvec::prelude::*;

pub enum ECCLevel {
    L,
    M,
    Q,
    H,
}

impl ECCLevel {
    fn get_error_correction_bitvec(&self) -> BitVec {
        match self {
            Self::L => return bitvec![0, 1],
            Self::M => return bitvec![0, 0],
            Self::Q => return bitvec![1, 1],
            Self::H => return bitvec![1, 0],
        }
    }
}

