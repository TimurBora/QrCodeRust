
use bitvec::{vec::BitVec, slice::BitSlice};
use reed_solomon::Encoder;
use crate::error_correction_level::ECCLevel;

extern crate reed_solomon_erasure;

pub struct ErrorCorrection {
    error_correction_level: ECCLevel,
}

impl ErrorCorrection {
    pub fn new(ecc_level: ECCLevel) -> Self {
        return Self { error_correction_level: ecc_level };
    }

    fn create_reed_solomon_encoder(num_error_correction_blocks: usize) -> Encoder {
        let reed_encoder: Encoder = Encoder::new(num_error_correction_blocks);

        return reed_encoder;
    }

    pub fn create_error_corrections_blocks(data: Vec<u8>, num_error_correction_blocks: usize) -> Vec<u8> {
        let reed_encoder: Encoder = Self::create_reed_solomon_encoder(num_error_correction_blocks);
        
        let data_copy = reed_encoder.encode(&data).to_vec();

        let error_correction_blocks: Vec<u8> = data_copy.iter().rev().take(num_error_correction_blocks).copied().rev().collect();
        
        return error_correction_blocks;
    }
}
