
use crate::module::Module;
use crate::qr_matrix::QrMatrix;
use crate::Mode;

pub struct Mask<'a> {
    matrix: &'a mut QrMatrix,
}

impl<'a> Mask<'a> {
    pub fn new(qr_matrix: &'a mut QrMatrix) -> Self {
        return Self { matrix: qr_matrix };
    }
    pub fn matrix_masking(&mut self) {
        for i in 0..21 {
            for j in 0..21 {
                if i % 2 == 0 {
                    let mut module: &mut Module = self.matrix.get_mut_module((i, j));
                    if !module.is_fun() {
                        module.flip_module();
                    }
                }
            }
        }
    }
}