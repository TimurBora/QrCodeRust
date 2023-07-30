

use bitvec::vec::BitVec;

use crate::qr_code_bitvec::QrCodeBitvec;
use crate::qr_matrix::QrMatrix;
use crate::finder_builder::FinderBuilder;
use crate::timing_builder::TimingBuilder;
use crate::info_blocks::InfoBlockBuilder;
use crate::numeric_data_operations::NumericToBinaryConverter;
use crate::data_mode::Mode;
use crate::bitvector_converter::BitVecConverter;
use crate::DataEncoder;
use crate::Mask;


pub struct QrBuilder {
    qr_matrix: QrMatrix,
}

impl QrBuilder {
    pub fn build(data:String, size: usize) -> Self {
        let mut qr_matrix: QrMatrix = Self::create_qr_matrix(size);

        let mut block_white: QrMatrix = QrMatrix::new(60);

        Self::add_functional_blocks_to_matrix(&mut qr_matrix);

        Self::encode_bitvec_to_matrix(data, &mut qr_matrix);

        Self::masking_qr_matrix(&mut qr_matrix);

        block_white.set_square(21, (20, 20), qr_matrix.get_modules());

        block_white.print_matrix();

        return Self { qr_matrix: qr_matrix };
    }

    fn create_qr_matrix(size: usize) -> QrMatrix {
        let qr_matrix: QrMatrix = QrMatrix::new(size);

        return qr_matrix;
    }

    fn add_functional_blocks_to_matrix(qr_matrix: &mut QrMatrix) {
        Self::add_finders_to_matrix(qr_matrix);
        Self::add_timing_to_matrix(qr_matrix);
        Self::add_info_block_to_matrix(qr_matrix);
    }

    fn add_finders_to_matrix(qr_matrix: &mut QrMatrix) {
        let mut finder_builder: FinderBuilder = FinderBuilder::new(qr_matrix);
        finder_builder.add_finders();
    }

    fn add_timing_to_matrix(qr_matrix: &mut QrMatrix) {
        let mut timing_builder = TimingBuilder::new(qr_matrix);
        timing_builder.add_timing_blocks();
    }

    fn add_info_block_to_matrix(qr_matrix: &mut QrMatrix) {
        let mut info_block_builder: InfoBlockBuilder = InfoBlockBuilder::new(qr_matrix);
        info_block_builder.add_to_matrix();
    }

    fn get_bitvec_to_encode(data: String) -> BitVec {
        let bitvector_converter: BitVecConverter = BitVecConverter::new(data);
        return bitvector_converter.get_result_bitvec();
    }

    fn encode_bitvec_to_matrix(data: String, qr_matrix: &mut QrMatrix) {
        let data_to_encode: BitVec = Self::get_bitvec_to_encode(data);

        let mut data_encoder: DataEncoder = DataEncoder::new(&data_to_encode, qr_matrix);
        data_encoder.encode_to_matrix();
    }

    fn masking_qr_matrix(qr_matrix: &mut QrMatrix) {
        let mut mask: Mask = Mask::new(qr_matrix);
        mask.matrix_masking();
    }
}