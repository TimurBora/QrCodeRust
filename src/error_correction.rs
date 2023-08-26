use reed_solomon::Encoder;

extern crate reed_solomon_erasure;

pub struct ErrorCorrection {}

impl ErrorCorrection {
    pub fn create_error_corrections_blocks(
        data: Vec<u8>,
        num_error_correction_blocks: usize,
    ) -> Vec<u8> {
        let reed_encoder: Encoder = Encoder::new(num_error_correction_blocks);

        let data_copy = reed_encoder.encode(&data).to_vec();

        let error_correction_blocks: Vec<u8> = data_copy
            .iter()
            .rev()
            .take(num_error_correction_blocks)
            .copied()
            .rev()
            .collect();

        return error_correction_blocks;
    }
}
