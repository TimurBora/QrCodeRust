
use crate::qr_matrix::QrMatrix;
use crate::Mode;

struct Mask<'a> {
    matrix: &'a mut QrMatrix,
}

impl<'a> Mask<'a> {
    fn matrix_masking(&mut self) {
        for i in 0..20 {
            for j in 0..20 {
                if i % 2 == 0 {
                    self.matrix.get_mut_module((i, j)).flip_module();
                }
            }
        }
    }
}