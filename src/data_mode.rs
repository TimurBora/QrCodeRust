

use bitvec::prelude::*;
use generic_matrix::Matrix;

use crate::qr_matrix::QrMatrix;
use crate::module::Module;

const MODE_SIZE: usize = 2;

pub enum Mode {
    Numeric,
}

impl Mode {
    pub fn get_bitvec(mode: &Mode) -> BitVec {
        match mode {
            Mode::Numeric => return bitvec![0, 0, 0, 1],
            _ => panic!("Stop"),
        }
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
                // 1 => vec_module.push(Module::Data(true)),
                // 0 => vec_module.push(Module::Data(false)),
                // _ => panic!("Opasnost"),
                _ => println!("{}", i),
            }
        }

        vec_module = vec![Module::Unknown; 4];

        return Matrix::from_vec(MODE_SIZE, MODE_SIZE, vec_module);
    }

    fn get_cordinate_mode(qr_matrix: &QrMatrix) -> (usize, usize) {
        return (qr_matrix.get_size() - MODE_SIZE - 1, qr_matrix.get_size() - MODE_SIZE - 1);
    }
}