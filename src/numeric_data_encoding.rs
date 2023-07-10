

use bitvec::vec;

use crate::data_mode::Mode;

pub struct NumericGroups {
    data: String,
}

impl NumericGroups {
    pub fn new(data: String) -> Self {
        return NumericGroups {data: data};
    }

    pub fn break_string_to_group(&self) -> Vec<u16> {
        let mut data: &String = self.get_data();
        let mut result_vector: Vec<u16> = Vec::new();

        for i in 0..data.len() / 3 {

            result_vector.push(self.convert_string_to_int(data[0 + i * 3..3 + i * 3].to_string()));
        }

        if data.len() % 3 != 0 {
            result_vector.push(self.get_last_elements(data).unwrap());
        }
        
        return result_vector;

    }

    fn convert_string_to_int(&self, string_data: String) -> u16 {
        let integer_data: u16 = string_data.parse().unwrap();
        return integer_data;
    }
    
    fn get_last_elements(&self, data: &String) -> Option<u16> {
        match data.len() % 3 {
            1 => return Some(self.convert_string_to_int(data[data.len() - 1..].to_string())),
            2 => return Some(self.convert_string_to_int(data[data.len() - 2..].to_string())),
            _ => None,
        }
    }

    fn get_data(&self) -> &String {
        return &self.data;
    }
}