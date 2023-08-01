

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
mod alphanumeric_data_operations;
mod byte_data_operations;

use byte_data_operations::ByteDataBitvec;
use error_correction::ErrorCorrection;
use qr_builder::QrBuilder;
use data_mode::Mode;
use bitvec::prelude::*;
use filling_data_vectors::add_bits_to_required_len;
use encode_data_to_matrix::DataEncoder;
use masking::Mask;
use qr_code_bitvec::QrCodeBitvec;
use alphanumeric_data_operations::AlphaNumericGroups;

fn append_to_bitvec(bitvec: &mut BitVec, integer: &u32, bit_len: usize) {
    bitvec.extend((0..bit_len).rev().map(|i| (integer >> i) & 1 != 0));
}

fn main() {
    let qr_builder: QrBuilder = QrBuilder::build("Пионири".to_string(), 21);
}