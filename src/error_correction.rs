
use bitvec::{vec::BitVec, slice::BitSlice};
use reed_solomon_erasure::galois_8::ReedSolomon;
use crate::error_correction_level::ECCLevel;

extern crate reed_solomon_erasure;

pub struct ErrorCorrection {
    error_correction_level: ECCLevel,
    data: Vec<Vec<u8>>,
}

impl ErrorCorrection {
    pub fn new(data: Vec<Vec<u8>>, ecc_level: ECCLevel) -> Self {
        return Self { error_correction_level: ecc_level, data: data };
    }

    fn create_reed_solomon_encoder() -> ReedSolomon {
        let data_blocks = 1;
        let error_correction_blocks = 7;

        let reed_encoder: ReedSolomon = ReedSolomon::new(data_blocks, error_correction_blocks).unwrap();

        return reed_encoder;
    }

    pub fn create_error_corrections_blocks(&mut self) {
        let reed_encoder: ReedSolomon = Self::create_reed_solomon_encoder();

        let mut data_copy = &mut self.data.clone();

        reed_encoder.encode(&mut data_copy).unwrap();

        println!("{:?}", data_copy);
    }
}