

use bitvec::prelude::*;
use generic_matrix::Matrix;

use crate::qr_matrix::QrMatrix;
use crate::module::Module;

use crate::append_to_bitvec;

const MODE_SIZE: usize = 2;

pub enum Mode {
    Numeric,
    Alphanumeric,
    Byte,
}

impl Mode {
    pub fn get_bitvec(mode: &Mode) -> BitVec {
        let mut bitvec: BitVec = BitVec::new();
        match mode {
            Mode::Numeric => append_to_bitvec(&mut bitvec, &1, 4),
            //Mode::Alphanumeric => return BitVec::from_vec(vec![0, 0, 1, 0]),
            //Mode::Byte => BitVec::from_vec(vec![0, 1, 0, 0]),
            _ => return bitvec,
        }
        return bitvec;
    }

    pub fn add_mode_to_qr_matrix(&self, qr_matrix: &mut QrMatrix) {
        let mut mode_matrix: Matrix<Module> = self.generate_matrix();

        qr_matrix.set_square(MODE_SIZE, Mode::get_cordinate_mode(qr_matrix), &mut mode_matrix);
    }

    pub fn generate_matrix(&self) -> Matrix<Module> {
        let bitvec: Vec<usize> = Mode::get_bitvec(self).into_vec();
        let mut vec_module: Vec<Module> = Vec::new();

        for i in bitvec.into_iter() {
            match i {
                1 => vec_module.insert(0, Module::Data(true)),
                0 => vec_module.insert(0, Module::Data(false)),
                _ => panic!("Opasnost"),
            }
        }

        let mut mode_matrix: Matrix<Module> = Matrix::from_vec(MODE_SIZE, MODE_SIZE, vec_module);

        return mode_matrix;
    }

    fn get_cordinate_mode(qr_matrix: &QrMatrix) -> (usize, usize) {
        return (qr_matrix.get_size() - MODE_SIZE, qr_matrix.get_size() - MODE_SIZE);
    }
}