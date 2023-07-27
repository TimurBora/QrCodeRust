

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
use qr_code_bitvec::QrCodeBitvec;

use crate::filling_data_vectors::add_terminator;

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

    let mut info_bitvec: BitVec = Mode::get_bitvec(&Mode::Numeric);

    info_bitvec.append(&mut get_bitvector_char_counter("3141".to_string()));

    let mut qr_code_bitvec: QrCodeBitvec = QrCodeBitvec::new();
    qr_code_bitvec.append_to_info_bitvec(&mut info_bitvec);
    
    let numeric_binary_convert: NumericToBinaryConverter = NumericToBinaryConverter::new("3141".to_string());
    let mut numeric_bitvector = numeric_binary_convert.merge_bit_vectors();

    qr_code_bitvec.append_to_data_bitvec(&mut numeric_bitvector);

    let qr_code_bitvec_len: usize = qr_code_bitvec.get_qr_code_bitvec_len();
    add_bits_to_required_len(qr_code_bitvec.get_mut_data_bitvec(), qr_code_bitvec_len);

    let mut to_ecc_bitvec: BitVec = qr_code_bitvec.merge_bitvec();
    let bytes_bitvector = to_ecc_bitvec.chunks(8);
    
    let mut byte_vector: Vec<u8> = Vec::new();

    for byte in bytes_bitvector.into_iter() {
        let mut reverse_byte: BitVec = BitVec::new();

        for x in byte.iter().rev() {
            reverse_byte.push(*x);
        }

        let integer: u8 = reverse_byte.load();
        byte_vector.push(integer);
    }  

    let data_with_ecc: Vec<u8> = ErrorCorrection::create_error_corrections_blocks(byte_vector, 7);

    let mut ecc_bitvec: BitVec = BitVec::new();
    for x in data_with_ecc.iter() {
        append_to_bitvec(&mut ecc_bitvec, &(*x as u32), 8);
    }

    qr_code_bitvec.append_to_ecc_bitvec(&mut ecc_bitvec);

    qr_code_bitvec.merge_bitvec();
    let data_to_encode: BitVec = qr_code_bitvec.merge_bitvec();

    let mut data_encoder: DataEncoder = DataEncoder::new(&data_to_encode, &mut qr_block);
    data_encoder.encode_to_matrix();

    let mut mask: Mask = Mask::new(&mut qr_block);
    mask.matrix_masking();

    block_white.set_square(21, (20, 20), qr_block.get_modules());

    block_white.print_matrix();
}