

use bitvec::prelude::*;

use crate::{module::Module, ZigZagIt::ZigZagIt, qr_matrix::QrMatrix};


#[derive(Debug)]
pub struct DataEncoder<'a> {
    data: &'a BitVec,
    qr_matrix: &'a mut QrMatrix,
}

impl<'a> DataEncoder<'a> {
    pub fn new(data: &'a BitVec, qr_matrix: &'a mut QrMatrix) -> Self {
        return DataEncoder { data: data, qr_matrix: qr_matrix};
    }

    pub fn encode_to_matrix(&mut self) {
        let zigzag: ZigZagIt = ZigZagIt::new(self.qr_matrix.get_size());
        let mut bitvector_index: usize = 0;
        for (row_cordinate, column_cordinate) in zigzag {
            let module: Module = *self.qr_matrix.get_module((row_cordinate, column_cordinate));
            if module.is_fun() { continue; }

            if bitvector_index > 205 {
                break;
            }

            self.qr_matrix.set_module((row_cordinate, column_cordinate), Module::Data(self.data[bitvector_index]));
            bitvector_index += 1;
        }
    }
}