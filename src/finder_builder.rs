

use std::ops::IndexMut;
use std::ops::Index;

use crate::qr_matrix::QrMatrix;

use crate::module::Module;

use crate::constants::FINDER_SIZE;
use crate::constants::FINDER_BLOCK;

use::generic_matrix;
use generic_matrix::Matrix;

pub struct FinderBuilder<'a> {
    matrix: &'a mut QrMatrix,   
}

impl<'a> FinderBuilder<'a> {
    pub fn new(matrix: &'a mut QrMatrix) -> Self {
        return FinderBuilder { matrix: matrix };
    }

    pub fn add_finders(&mut self) {
        let cordinates: [(usize, usize, usize); 3] = self.get_finder_cordinate();

        for cordinate in cordinates {
            self.add_finder(cordinate)
        }
    
    }

    fn add_finder(&mut self, cordinate: (usize, usize, usize)) {
        let mut finder_matrix: Matrix<Module> = Matrix::from_vec(FINDER_SIZE, FINDER_SIZE, FinderBuilder::generate_finder());

        let rotate_num: usize = cordinate.2;
        FinderBuilder::rotate_finder(&mut finder_matrix, rotate_num);

        let start_height: usize = cordinate.0;
        let start_width: usize = cordinate.1;

        for i in 0..FINDER_SIZE {
            for j in 0..FINDER_SIZE {
                let mut main_element: &mut Module = self.matrix.get_modules().index_mut((i + start_height, j + start_width));
                *main_element = *finder_matrix.index_mut((i, j));
            }
        }
    }

    fn get_finder_cordinate(&mut self) -> [(usize, usize, usize); 3] {
        return [(0, 0, 0),
                (0, self.matrix.get_modules().column() - FINDER_SIZE as usize, 1), 
                (self.matrix.get_modules().row() - FINDER_SIZE as usize, 0, 3)];
    }

    fn generate_finder() -> Vec<Module> {
        let mut matrix_vector: Vec<Module> = Vec::new();
        for i in 0..FINDER_SIZE {
            for j in 0..FINDER_SIZE {
                match FINDER_BLOCK[i][j] {
                    0 => matrix_vector.push(Module::Function(false)),
                    1 => matrix_vector.push(Module::Function(true)),
                    _ => panic!("ОШИБКА"),
                }
            }
        }
        
        return matrix_vector;
    }

    fn rotate_finder(finder_matrix: &mut Matrix<Module>, rotate_num: usize) -> &mut Matrix<Module> {
        let mut temp: Vec<Vec<Module>> = vec![vec![Module::Unknown; FINDER_SIZE]; FINDER_SIZE];
        for _ in 0..rotate_num {
            for i in 0..FINDER_SIZE {
                for j in 0..FINDER_SIZE {
                    temp[j][FINDER_SIZE - i - 1] = *finder_matrix.index((i, j));
                }
            }

            for i in 0..FINDER_SIZE {
                for j in 0..FINDER_SIZE {
                    *finder_matrix.index_mut((i, j)) = temp[i][j];
                }
            }
        }
        return finder_matrix;
    }
}