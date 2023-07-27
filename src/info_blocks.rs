

use bitvec::prelude::*;

use crate::error_correction_level::ECCLevel;

use crate::append_to_bitvec;
use crate::masking::Mask;
use crate::module::Module;
use crate::qr_matrix::QrMatrix;

static FORMAT_INFO: [[u16; 4]; 8] = [
    [0b111011111000100, 0b101010000010010, 0b011010101011111, 0b001011010001001],
    [0b111001011110011, 0b101000100100101, 0b011000001101000, 0b001001110111110],
    [0b111110110101010, 0b101111001111100, 0b011111100110001, 0b001110011100111],
    [0b111100010011101, 0b101101101001011, 0b011101000000110, 0b001100111010000],
    [0b110011000101111, 0b100010111111001, 0b010010010110100, 0b000011101100010],
    [0b110001100011000, 0b100000011001110, 0b010000110000011, 0b000001001010101],
    [0b110110001000001, 0b100111110010111, 0b010111011011010, 0b000110100001100],
    [0b110100101110110, 0b100101010100000, 0b010101111101101, 0b000100000111011]
];


pub struct InfoBlockBuilder<'a> {
    qr_matrix: &'a mut QrMatrix,
}

impl<'a> InfoBlockBuilder<'a> {
    pub fn new(qr_matrix: &'a mut QrMatrix) -> Self {
        return Self { qr_matrix: qr_matrix };
    }

    pub fn add_to_matrix(&mut self) {
        self.add_to_first_finder();
        self.add_to_other_finders();
    }

    fn add_to_first_finder(&mut self) {
        let format_bitvec: BitVec = self.generate_bitvec();

        let mut bitvec_index: usize = 0;
        for column in 0..9 {
            if column == 6 { continue; }

            self.qr_matrix.set_module((8, column), Module::Function(format_bitvec[bitvec_index]));
            bitvec_index += 1;
        }

        for row in (0..8).rev() {
            if row == 6 { continue; }

            self.qr_matrix.set_module((row, 8), Module::Function(format_bitvec[bitvec_index]));
            bitvec_index += 1;
        }
    }

    fn add_to_other_finders(&mut self) {
        let format_bitvec: BitVec = self.generate_bitvec();

        let mut bitvec_index: usize = 0;

        for row in 0..8 {
            if row == 7 {
                self.qr_matrix.set_module((21 - 8, 8), Module::Function(true));
                break;
            }

            self.qr_matrix.set_module((20 - row, 8), Module::Function(format_bitvec[bitvec_index]));
            bitvec_index += 1;
        }

        for column in (0..8).rev() {
            self.qr_matrix.set_module((8, 20 - column), Module::Function(format_bitvec[bitvec_index]));
            bitvec_index += 1;
        }
    }

    fn generate_bitvec(&self) -> BitVec {
        let mut bitvec: BitVec = BitVec::new();
        let format_integer: u32 = FORMAT_INFO[0][0] as u32;

        append_to_bitvec(&mut bitvec, &format_integer, 15);
        return bitvec;
    }
}
