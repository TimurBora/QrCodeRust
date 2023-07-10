

use bitvec::prelude::*;

use crate::data_mode::Mode;

struct NumericGroups {
    data: String,
}

impl NumericGroups {
    fn new(data: String) -> Self {
        return NumericGroups {data: data};
    }

    fn break_string_to_group(&self) -> Vec<u16> {
        let data: &String = self.get_data();
        let mut result_vector: Vec<u16> = Vec::new();

        for i in 0..data.len() / 3 {

            result_vector.push(self.convert_string_to_int(data[0 + i * 3..3 + i * 3].to_string()));
        }

        if data.len() % 3 != 0 {
            let num_last_elements: usize = data.len() % 3;
            let last_elements_vector: String = data[data.len() - num_last_elements..].to_string();
            result_vector.push(self.convert_string_to_int(last_elements_vector));
        }
        
        return result_vector;

    }

    fn convert_string_to_int(&self, string_data: String) -> u16 {
        let integer_data: u16 = string_data.parse().unwrap();
        return integer_data;
    }

    fn get_data(&self) -> &String {
        return &self.data;
    }
}

struct NumericToBinaryConverter {
    integer_group_vector: Vec<u16>,
}

impl NumericToBinaryConverter {
    fn new(data: String) -> Self {
        let group_vector: Vec<u16> = NumericToBinaryConverter::get_group_vector(data);
        return NumericToBinaryConverter { integer_group_vector: group_vector };
    }

    // fn get_binary_vectors(&self) -> Vec<BitVec<u16, Msb0>> {

    // }

    fn generate_bitvec(binary: u16) -> BitVec<u16, Msb0> {
        let bitvector: BitVec<u16, Msb0> = binary.view_bits::<Msb0>().to_bitvec();
        return bitvector;
    }

    fn convert_to_binary(decimal: u16) -> u16 {
        let binary: u16 = format!("{:b}", decimal).parse().unwrap();
        return binary;
    }

    fn get_group_vector(data: String) -> Vec<u16> {
        let numeric_groups: NumericGroups = NumericGroups::new(data);
        return numeric_groups.break_string_to_group(); 
    }
}