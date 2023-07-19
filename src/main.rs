

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

    finder_builder.add_finders();

    let mut timing_builder: TimingBuilder = TimingBuilder::new(&mut qr_block);
    timing_builder.add_timing_blocks();

    let mut bitvec: BitVec = Mode::get_bitvec(&Mode::Numeric);
    bitvec.reserve(800);

    bitvec.append(&mut get_bitvector_char_counter("0".to_string()));
    
    let numeric_binary_convert: NumericToBinaryConverter = NumericToBinaryConverter::new("0".to_string());
    let mut numeric_bitvector = numeric_binary_convert.merge_bit_vectors();

    add_bits_to_required_len(&mut numeric_bitvector);

    bitvec.append(&mut numeric_bitvector);

    let mut data_encoder: DataEncoder = DataEncoder::new(&bitvec, &mut qr_block);
    data_encoder.encode_to_matrix();

    //qr_block.print_matrix();

    let mut data_block = vec![vec![1, 2, 3, 4, 5, 6, 7, 8]];

    // Создание и заполнение вектора для исправления ошибок (error correction blocks)
    let mut error_correction_blocks = vec![vec![0; 8]; 7];

    data_block.append(&mut error_correction_blocks);

    let mut error_correction: ErrorCorrection = ErrorCorrection::new(data_block, error_correction_level::ECCLevel::H);
    error_correction.create_error_corrections_blocks();
}