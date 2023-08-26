mod bitvector_converter;
mod byte_data_operations;
mod char_counter_builder;
mod constants;
mod data_mode;
mod encode_data_to_matrix;
mod error_correction;
mod filling_data_vectors;
mod finder_builder;
mod info_blocks;
mod masking;
mod module;
mod numeric_data_operations;
mod qr_builder;
mod qr_code_bitvec;
mod qr_matrix;
mod timing_builder;
mod zig_zag_it;

use bitvec::prelude::*;
use encode_data_to_matrix::DataEncoder;
use error_correction::ErrorCorrection;
use filling_data_vectors::add_bits_to_required_len;
use masking::Mask;
use qr_builder::QrBuilder;
use qr_code_bitvec::QrCodeBitvec;

fn append_to_bitvec(bitvec: &mut BitVec, integer: &u32, bit_len: usize) {
    bitvec.extend((0..bit_len).rev().map(|i| (integer >> i) & 1 != 0));
}

fn main() {
    QrBuilder::build("HelloWorld".to_string(), 21);
}
