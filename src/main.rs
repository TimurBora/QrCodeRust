

mod qr_matrix;
mod finder_builder;
mod timing_builder;
mod constants;
mod module;
mod data_mode;
mod numeric_data_operations;
mod filling_data_vectors;
mod encode_data_to_matrix;
mod ZigZagIt;
mod char_counter_builder;
mod error_correction_level;
mod error_correction;
mod info_blocks;
mod masking;
mod qr_code_bitvec;
mod qr_builder;
mod bitvector_converter;

use error_correction::ErrorCorrection;
use qr_builder::QrBuilder;
use qr_matrix::QrMatrix;
use finder_builder::FinderBuilder;
use timing_builder::TimingBuilder;
use data_mode::Mode;
use numeric_data_operations::NumericToBinaryConverter;
use bitvec::prelude::*;
use filling_data_vectors::add_bits_to_required_len;
use encode_data_to_matrix::DataEncoder;
use char_counter_builder::get_bitvector_char_counter;
use info_blocks::InfoBlockBuilder;
use masking::Mask;
use qr_code_bitvec::QrCodeBitvec;

fn append_to_bitvec(bitvec: &mut BitVec, integer: &u32, bit_len: usize) {
    bitvec.extend((0..bit_len).rev().map(|i| (integer >> i) & 1 != 0));
}

fn main() {
    let qr_builder: QrBuilder = QrBuilder::build("123123".to_string(), 21);
}