

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

pub struct NumericToBinaryConverter {
    integer_group_vector: Vec<u16>,
}

impl NumericToBinaryConverter {
    pub fn new(data: String) -> Self {
        let group_vector: Vec<u16> = NumericToBinaryConverter::get_group_vector(data);
        return NumericToBinaryConverter { integer_group_vector: group_vector };
    }

    pub fn merge_bit_vectors(&self) -> BitVec {
        let mut bit_vector: BitVec = BitVec::new();
        for integer in self.integer_group_vector.iter() {
            let mut integer_bitvec: BitVec = NumericToBinaryConverter::generate_bitvec(*integer);
            bit_vector.append(&mut integer_bitvec);
        }

        for x in bit_vector.iter() {
            print!("{} ", if *x { "1" } else { "0" });
        }

        return bit_vector;
    }

    fn generate_bitvec(integer: u16) -> BitVec {
        let bit_len = |num: u16| {
            if num > 99 {
                return 10;
            }
            else if num > 9 {
                return 7;
            }
            else {
                return 4;
            }
        };

        let mut bitvector: BitVec = BitVec::new();
        bitvector.reserve(bit_len(integer));

        bitvector.extend((0..bit_len(integer)).rev().map(|i| (integer >> i) & 1 != 0));

        return bitvector;
    }

    fn get_group_vector(data: String) -> Vec<u16> {
        let numeric_groups: NumericGroups = NumericGroups::new(data);
        return numeric_groups.break_string_to_group(); 
    }
}