
use std::ops::{IndexMut, Index};

use::generic_matrix;
use generic_matrix::Matrix;

const SCAN_BLOCK: [[i32; 8]; 8] = [[1, 1, 1, 1, 1, 1, 1, 0],
                                   [1, 0, 0, 0, 0, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 0, 0, 0, 0, 1, 0],
                                   [1, 1, 1, 1, 1, 1, 1, 0],
                                   [0, 0, 0, 0, 0, 0, 0, 0]];

#[derive(Clone, Copy, PartialEq)]
enum Module {
    Unknown,
    Reserved,
    Data(bool),
    Function(bool),
}

impl Module {

}

struct QrMatrix {
    size: usize,
    modules: Matrix<Module>,
}

impl QrMatrix {
    fn new(size: usize) -> Self {
        QrMatrix { size: size, modules: Matrix::from_vec(size, size, vec![Module::Unknown; size * size]) }
    }

    fn set_rect(&mut self, cordinate1: (usize, usize), cordinate2: (usize, usize), module: Module) {
        let start_width: usize = cordinate1.1;
        let start_height: usize = cordinate1.0;

        let width: usize = get_width(&cordinate1, &cordinate2);
        let height: usize = get_height(&cordinate1, &cordinate2);

        for i in start_height..height {
            for j in start_width..width {
                let mut _module: &mut Module = self.modules.index_mut((i, j));
                *_module = module;
            }
        }
    }

    fn set_square(&mut self, size: usize, cordinate: (usize, usize), module: Module) {
        let start_width: usize = cordinate.1;
        let start_height: usize = cordinate.0;

        for i in start_height..size {
            for j in start_width..size {
                let mut _module: &mut Module = self.modules.index_mut((i, j));
                *_module = module;
            }
        }
    }

    fn print_matrix(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.modules.index((i, j)) {
                    Module::Unknown => print!("-"),
                    Module::Function(true) => print!("1"),
                    Module::Function(false) => print!("0"),
                    _ => print!("0"),
                }
            }
            println!("");
        }
    }
}

struct FinderBuilder<'a> {
    matrix: &'a mut QrMatrix,   
}

impl<'a> FinderBuilder<'a> {
    fn add_finders(&mut self) {
        let cordinates: [(usize, usize, usize); 3] = self.get_finder_cordinate();

        for cordinate in cordinates {
            self.add_finder(&cordinate)
        }
    
    }

    fn add_finder(&mut self, cordinate: &(usize, usize, usize)) {
        let mut finder_matrix: Matrix<Module> = Matrix::from_vec(8, 8, FinderBuilder::generate_finder());

        let rotate_num = cordinate.2;
        FinderBuilder::rotate_finder(&mut finder_matrix, rotate_num);

        let start_height: usize = cordinate.0;
        let start_width: usize = cordinate.1;

        for i in 0..8 {
            for j in 0..8 {
                let mut main_element: &mut Module = self.matrix.modules.index_mut((i + start_height, j + start_width));
                *main_element = *finder_matrix.index_mut((i, j));  
            }
        }
    }

    fn get_finder_cordinate(&self) -> [(usize, usize, usize); 3] {
        return [(0, 0, 0),
                (0, self.matrix.modules.column() - 8 as usize, 1), 
                (self.matrix.modules.row() - 8 as usize, 0, 3)];
    }

    fn generate_finder() -> Vec<Module> {
        let mut matrix_vector: Vec<Module> = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if SCAN_BLOCK[i][j] == 1 {
                    matrix_vector.push(Module::Function(true));
                }
                else {
                    matrix_vector.push(Module::Function(false));
                }
            }
        }
        
        return matrix_vector;
    }

    fn rotate_finder(finder_matrix: &mut Matrix<Module>, rotate_num: usize) -> &mut Matrix<Module> {
        let n: usize = 8;

        let mut temp: Vec<Vec<Module>> = vec![vec![Module::Unknown; n]; n];
        for _ in 0..rotate_num {
            for i in 0..n {
                for j in 0..n {
                    temp[j][n - i - 1] = *finder_matrix.index((i, j));
                }
            }

            for i in 0..n {
                for j in 0..n {
                    *finder_matrix.index_mut((i, j)) = temp[i][j];
                }
            }
        }
        return finder_matrix;
    }
}

fn get_width(cordinate1: &(usize, usize), cordinate2: &(usize, usize)) -> usize {
    let width: usize = cordinate2.1 - cordinate1.1 + 1;

    return width;
}

fn get_height(cordinate1: &(usize, usize), cordinate2: &(usize, usize)) -> usize {
    let height: usize = cordinate2.0 - cordinate1.0 + 1 ;

    return height;
}


fn main() {
    let mut qr_block: QrMatrix = QrMatrix::new(28);
    let mut finder_builder: FinderBuilder = FinderBuilder { matrix: &mut qr_block };

    finder_builder.add_finders();

    qr_block.print_matrix();
}