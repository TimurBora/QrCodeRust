

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

use error_correction::ErrorCorrection;
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

fn print_bitvec(bitvec: &BitVec) {
    for i in bitvec {
        print!("{} ", if *i {"1"} else {"0"});
    }
    println!("");
}

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

    let mut block_white: QrMatrix = QrMatrix::new(60);

    finder_builder.add_finders();

    let mut timing_builder: TimingBuilder = TimingBuilder::new(&mut qr_block);
    timing_builder.add_timing_blocks();

    let mut info_blocks_builder: InfoBlockBuilder = InfoBlockBuilder::new(&mut qr_block);
    info_blocks_builder.add_to_matrix();

    let mut bitvec: BitVec = Mode::get_bitvec(&Mode::Numeric);
    bitvec.reserve(800);

    bitvec.append(&mut get_bitvector_char_counter("123".to_string()));
    
    let numeric_binary_convert: NumericToBinaryConverter = NumericToBinaryConverter::new("123".to_string());
    let mut numeric_bitvector = numeric_binary_convert.merge_bit_vectors();

    add_bits_to_required_len(&mut numeric_bitvector);

    let bytes_bitvector = numeric_bitvector.split(|pos, _bits| pos == 8);
    
    let mut byte_vector: Vec<u8> = Vec::new();

    for byte in bytes_bitvector.into_iter() {
        let integer: u8 = byte.load();
        byte_vector.push(integer);
    }

    let data_with_ecc: Vec<u8> = ErrorCorrection::create_error_corrections_blocks(byte_vector);

    for integer in data_with_ecc.iter() {
        append_to_bitvec(&mut bitvec, &(*integer as u32), 8);
    }

    let mut data_encoder: DataEncoder = DataEncoder::new(&bitvec, &mut qr_block);
    data_encoder.encode_to_matrix();

    let mut mask: Mask = Mask::new(&mut qr_block);
    mask.matrix_masking();

    block_white.set_square(21, (20, 20), qr_block.get_modules());

    block_white.print_matrix();
}