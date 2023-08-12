//Не закончено ещё(точнее пока что не начато)

use bitvec::prelude::*;
use crate::append_to_bitvec;

pub struct AlphaNumericGroups {
    data: String,
}

impl AlphaNumericGroups {
    pub fn new(data: String) -> Self {
        return AlphaNumericGroups {data: data};
    }

    pub fn break_string_to_group(&self) -> Vec<String> {
        let data: &String = self.get_data();
        let mut result_vector: Vec<String> = Vec::new();

        for i in 0..data.len() / 2 {
            result_vector.push(data[0 + i * 2..2 + i * 2].to_string());
        }

        if data.len() % 2 != 0 {
            let last_elements_vector: String = data[data.len() - 1..].to_string();
            result_vector.push(last_elements_vector);
        }
        
        return result_vector;
    }

    fn convert_chars_from_group(&self) -> Vec<(u32, u32)> {
        let data_groups: Vec<String> = self.break_string_to_group();

        let mut integer_vec: Vec<(u32, u32)> = Vec::new();

        for group in data_groups {
            
        }

        return integer_vec;
    }

    fn get_data(&self) -> &String {
        return &self.data;
    }
}

fn convert_alphanumeric(char: char) -> u32 {
    match char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'I' => 18,
        'J' => 19,
        'K' => 20,
        'L' => 21,
        'M' => 22,
        'N' => 23,
        'O' => 24,
        'P' => 25,
        'Q' => 26,
        'R' => 27,
        'S' => 28,
        'T' => 29,
        'U' => 30,
        'V' => 31,
        'W' => 32,
        'X' => 33,
        'Y' => 34,
        'Z' => 35,
        ' ' => 36,
        '$' => 37,
        '%' => 38,
        '*' => 39,
        '+' => 40,
        '-' => 41,
        '.' => 42,
        '/' => 43,
        ':' => 44,
        _ => panic!("Unsupported alphanumeric '{}'", char),
    }
}
