
use bitvec::{vec::BitVec, slice::BitSlice};
use reed_solomon::Encoder;
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

    fn create_reed_solomon_encoder() -> Encoder {
        let error_correction_blocks = 7;

        let reed_encoder: Encoder = Encoder::new(error_correction_blocks);

        return reed_encoder;
    }

    pub fn create_error_corrections_blocks(data: Vec<u8>) -> Vec<u8> {
        let reed_encoder: Encoder = Self::create_reed_solomon_encoder();
        
        let data_copy = reed_encoder.encode(&data).to_vec();
        
        return data_copy;
    }
}
