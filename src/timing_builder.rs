use crate::module::Module;
use crate::qr_matrix::QrMatrix;

use crate::constants::FINDER_SIZE;

use ::generic_matrix;
use generic_matrix::Matrix;

use std::ops::IndexMut;

pub struct TimingBuilder<'a> {
    matrix: &'a mut QrMatrix,
}

impl<'a> TimingBuilder<'a> {
    pub fn new(matrix: &'a mut QrMatrix) -> Self {
        TimingBuilder { matrix: matrix }
    }

    pub fn add_timing_blocks(&mut self) {
        let mut timing_matrix: Matrix<Module> = self.generate_timing_block();

        let cordinates: [(usize, usize); 2] = TimingBuilder::get_timing_cordinates();
        let size_timing: usize = self.get_size_timing();

        for i in 0..size_timing {
            let timing_element: &mut Module = timing_matrix.index_mut((i, 0));

            self.matrix
                .set_module((cordinates[0].0, i + FINDER_SIZE), *timing_element);
            self.matrix
                .set_module((i + FINDER_SIZE, cordinates[1].1), *timing_element);
        }
    }

    fn generate_timing_block(&self) -> Matrix<Module> {
        let mut timing_vector: Vec<Module> = Vec::new();

        let size_timing: usize = self.get_size_timing();

        let mut boolean_timing: bool = true;

        for _ in 0..size_timing {
            timing_vector.push(Module::Function(boolean_timing));

            boolean_timing = !boolean_timing;
        }

        let timing_block: Matrix<Module> = Matrix::from_vec(size_timing, 1, timing_vector);

        return timing_block;
    }

    fn get_timing_cordinates() -> [(usize, usize); 2] {
        return [(6, 8), (8, 6)];
    }

    fn get_size_timing(&self) -> usize {
        return self.matrix.get_size() - 2 * FINDER_SIZE;
    }
}
