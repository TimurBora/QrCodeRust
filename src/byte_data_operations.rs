

use bitvec::prelude::*;
use crate::append_to_bitvec;

pub struct ByteDataBitvec {
    data: String,
}

impl ByteDataBitvec{
    pub fn new(data: String) -> Self {
        return Self {data: data};
    }

    pub fn create_bitvec(&self) -> BitVec {
        let data_bytes: &[u8] = self.to_bytes();

        let mut utf8_bitvec: BitVec = BitVec::new();

        for byte in data_bytes {
            append_to_bitvec(&mut utf8_bitvec, &(*byte as u32), 8)
        }

        return utf8_bitvec;
    }

    fn to_bytes(&self) -> &[u8] {
        let data_bytes: &[u8] = self.data.as_bytes();
        
        return data_bytes;
    }
}