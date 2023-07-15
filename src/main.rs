

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

use qr_matrix::QrMatrix;
use finder_builder::FinderBuilder;
use timing_builder::TimingBuilder;
use data_mode::Mode;
use numeric_data_operations::NumericToBinaryConverter;
use bitvec::prelude::*;
use filling_data_vectors::add_bits_to_required_len;
use encode_data_to_matrix::DataEncoder;

fn append_to_bitvec(bitvec: &mut BitVec, integer: &u32, bit_len: usize) {
    bitvec.extend((0..bit_len).rev().map(|i| (integer >> i) & 1 != 0));
}

fn get_width(cordinate1: &(usize, usize), cordinate2: &(usize, usize)) -> usize {
    let width: usize = cordinate2.1 - cordinate1.1 + 1;

    return width;
}

fn get_height(cordinate1: &(usize, usize), cordinate2: &(usize, usize)) -> usize {
    let height: usize = cordinate2.0 - cordinate1.0 + 1 ;

    return height;
}


fn main() {
    let mut qr_block: QrMatrix = QrMatrix::new(21);
    let mut finder_builder: FinderBuilder = FinderBuilder::new(&mut qr_block);

    finder_builder.add_finders();

    let mut timing_builder: TimingBuilder = TimingBuilder::new(&mut qr_block);
    timing_builder.add_timing_blocks();

    // let mode: Mode = Mode::Byte;
    // mode.add_mode_to_qr_matrix(&mut qr_block);

    let numeric_binary_convert: NumericToBinaryConverter = NumericToBinaryConverter::new("123123".to_string());
    let mut numeric_bitvector = numeric_binary_convert.merge_bit_vectors();

    add_bits_to_required_len(&mut numeric_bitvector);

    let mut data_encoder: DataEncoder = DataEncoder::new(numeric_bitvector, &mut qr_block);
    data_encoder.encode_to_matrix();

    qr_block.print_matrix();
}