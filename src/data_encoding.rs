

use crate::data_mode::Mode;

pub struct NumericEncode {
    data: String,
}

impl NumericEncode {
    pub fn new(data: String) -> Self {
        return NumericEncode {data: data};
    }

    pub fn break_string_to_group(&self) -> Vec<Vec<u16>> {
        let mut data_vector: Vec<u16> = self.convert_data_to_vector();
        let mut result_vector: Vec<Vec<u16>> = Vec::new();

        for i in 0..data_vector.len() / 3 {
            let mut vector: Vec<u16> = Vec::new();

            for j in 0 + i * 3..3 + i * 3 {
                vector.push(data_vector[j]);
            }

            result_vector.push(vector);
        }

        match data_vector.len() % 3 {
            1 => result_vector.push(vec![data_vector[data_vector.len() - 1]]),
            2 => result_vector.push(data_vector[data_vector.len()-2..].to_vec()),
            _ => (),
        }



        for x in &result_vector {
            for y in x {
                print!("{} ", y);
            }
            println!("");
        }

        return result_vector;

    }

    fn convert_char_to_int(&self, string_data: char) -> u16 {
        let integer_data: u16 = string_data as u16 - 0x30;
        return integer_data;
    }

    fn convert_data_to_vector(&self) -> Vec<u16> {
        let data: &String = self.get_data();
        let mut data_vector: Vec<u16> = Vec::new();

        for element in data.chars() {
            data_vector.push(self.convert_char_to_int(element));
        }

        return data_vector;
    }

    fn get_data(&self) -> &String {
        return &self.data;
    }
}