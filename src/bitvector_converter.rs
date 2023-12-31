use bitvec::prelude::*;

use crate::add_bits_to_required_len;
use crate::append_to_bitvec;
use crate::byte_data_operations::ByteDataBitvec;
use crate::char_counter_builder::get_bitvector_char_counter;
use crate::numeric_data_operations::NumericToBinaryConverter;

use crate::data_mode::Mode;
use crate::ErrorCorrection;
use crate::QrCodeBitvec;

pub struct DataBitvec {
    data: String,
}

impl DataBitvec {
    pub fn new(data: String) -> Self {
        return DataBitvec { data: data };
    }

    pub fn get_result_bitvec(&self) -> BitVec {
        let mut qr_code_bitvec: QrCodeBitvec = QrCodeBitvec::new();

        self.append_info_bitvec(&mut qr_code_bitvec);
        self.append_data_bitvec(&mut qr_code_bitvec);

        Self::add_bits_to_required_len(&mut qr_code_bitvec);

        self.append_ecc_bitvec(&mut qr_code_bitvec);

        return qr_code_bitvec.merge_bitvec();
    }

    fn append_data_bitvec(&self, qr_code_bitvec: &mut QrCodeBitvec) {
        let mode: Mode = Mode::Byte;

        let mut data_bitvec: BitVec = self.convert_with_mode(mode);

        qr_code_bitvec.append_to_data_bitvec(&mut data_bitvec);
    }

    fn append_info_bitvec(&self, qr_code_bitvec: &mut QrCodeBitvec) {
        let mode: Mode = Mode::Byte;

        let mut info_bitvec: BitVec = Mode::get_bitvec(&mode);
        info_bitvec.append(&mut get_bitvector_char_counter(&self.data, &mode));

        qr_code_bitvec.append_to_info_bitvec(&mut info_bitvec);
    }

    fn append_ecc_bitvec(&self, qr_code_bitvec: &mut QrCodeBitvec) {
        let code_names: Vec<u8> = Self::get_code_names(qr_code_bitvec);

        let mut ecc_bitvec: BitVec = Self::convert_ecc_codenames_to_bitvec(code_names);

        qr_code_bitvec.append_to_ecc_bitvec(&mut ecc_bitvec);
    }

    fn convert_ecc_codenames_to_bitvec(code_names: Vec<u8>) -> BitVec {
        let mut ecc_bitvec: BitVec = BitVec::new();
        for integer in code_names.iter() {
            append_to_bitvec(&mut ecc_bitvec, &(*integer as u32), 8);
        }

        return ecc_bitvec;
    }

    fn get_code_names(qr_code_bitvec: &mut QrCodeBitvec) -> Vec<u8> {
        let to_ecc_bitvec: BitVec = qr_code_bitvec.merge_bitvec();
        let bytes_bitvector = to_ecc_bitvec.chunks(8);

        let mut byte_vector: Vec<u8> = Vec::new();

        for byte in bytes_bitvector.into_iter() {
            let mut reverse_byte: BitVec = BitVec::new();

            for x in byte.iter().rev() {
                reverse_byte.push(*x);
            }

            let integer: u8 = reverse_byte.load();
            byte_vector.push(integer);
        }
        dbg!(&byte_vector);

        let data_with_ecc: Vec<u8> =
            ErrorCorrection::create_error_corrections_blocks(byte_vector, 7);

        return data_with_ecc;
    }

    fn convert_with_mode(&self, mode: Mode) -> BitVec {
        match mode {
            Mode::Numeric => Self::numeric_binary_convert(&self.data),
            Mode::Byte => Self::byte_binary_convery(&self.data),
            _ => panic!("Opasno!"),
        }
    }

    fn numeric_binary_convert(data: &String) -> BitVec {
        let numeric_binary_convert: NumericToBinaryConverter =
            NumericToBinaryConverter::new(data.to_string());
        let numeric_bitvector = numeric_binary_convert.merge_bit_vectors();

        return numeric_bitvector;
    }

    fn byte_binary_convery(data: &String) -> BitVec {
        let utf8_binary_converter: ByteDataBitvec = ByteDataBitvec::new(data.to_string());
        return utf8_binary_converter.create_bitvec();
    }

    fn add_bits_to_required_len(bitvector: &mut QrCodeBitvec) {
        let bitvector_len: usize = bitvector.get_qr_code_bitvec_len();

        add_bits_to_required_len(bitvector.get_mut_data_bitvec(), bitvector_len);
    }
}
