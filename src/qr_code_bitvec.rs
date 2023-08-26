use bitvec::vec::BitVec;

pub struct QrCodeBitvec {
    info_bitvec: BitVec,
    data_bitvec: BitVec,
    ecc_bitvec: BitVec,
}

impl QrCodeBitvec {
    pub fn new() -> Self {
        let null_bitvec: BitVec = BitVec::new();
        return Self {
            info_bitvec: null_bitvec.clone(),
            data_bitvec: null_bitvec.clone(),
            ecc_bitvec: null_bitvec.clone(),
        };
    }

    pub fn merge_bitvec(&mut self) -> BitVec {
        let mut merged_bitvec: BitVec = BitVec::new();

        merged_bitvec.append(&mut self.info_bitvec.clone());
        merged_bitvec.append(&mut self.data_bitvec.clone());
        merged_bitvec.append(&mut self.ecc_bitvec.clone());

        return merged_bitvec;
    }

    pub fn get_qr_code_bitvec_len(&self) -> usize {
        return self.info_bitvec.len() + self.data_bitvec.len() + self.ecc_bitvec.len();
    }

    pub fn append_to_info_bitvec(&mut self, append_bitvec: &mut BitVec) {
        Self::append_to_bitvec(&mut self.info_bitvec, append_bitvec);
    }

    pub fn append_to_data_bitvec(&mut self, append_bitvec: &mut BitVec) {
        Self::append_to_bitvec(&mut self.data_bitvec, append_bitvec);
    }

    pub fn append_to_ecc_bitvec(&mut self, append_bitvec: &mut BitVec) {
        Self::append_to_bitvec(&mut self.ecc_bitvec, append_bitvec);
    }

    fn append_to_bitvec(main_bitvec: &mut BitVec, append_bitvec: &mut BitVec) {
        main_bitvec.append(append_bitvec);
    }

    pub fn get_mut_data_bitvec(&mut self) -> &mut BitVec {
        return &mut self.data_bitvec;
    }
}
