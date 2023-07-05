
use std::ops::{IndexMut, Index};

use::generic_matrix;
use generic_matrix::Matrix;

const FINDER_BLOCK: [[i32; FINDER_SIZE]; FINDER_SIZE] = [[1, 1, 1, 1, 1, 1, 1, 0],
                                   [1, 0, 0, 0, 0, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 1, 1, 1, 0, 1, 0],
                                   [1, 0, 0, 0, 0, 0, 1, 0],
                                   [1, 1, 1, 1, 1, 1, 1, 0],
                                   [0, 0, 0, 0, 0, 0, 0, 0]];

const FINDER_SIZE: usize = 8;

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
                    Module::Reserved => print!("0"),
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
                let mut main_element: &mut Module = self.matrix.modules.index_mut((i + start_height, j + start_width));
                *main_element = *finder_matrix.index_mut((i, j));  
            }
        }
    }

    fn get_finder_cordinate(&self) -> [(usize, usize, usize); 3] {
        return [(0, 0, 0),
                (0, self.matrix.modules.column() - FINDER_SIZE as usize, 1), 
                (self.matrix.modules.row() - FINDER_SIZE as usize, 0, 3)];
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

struct TimingBuilder<'a> {
    matrix: &'a mut QrMatrix, 
}

impl<'a> TimingBuilder<'a> {
    fn add_timing_blocks(&mut self) {
        let mut timing_matrix: Matrix<Module> = self.generate_timing_block();

        let cordinates: [(usize, usize); 2] = TimingBuilder::get_timing_cordinates();
        let size_timing: usize = self.get_size_timing();

        for i in 0..size_timing {
            let timing_element: &mut Module = timing_matrix.index_mut((i, 0));

            *self.matrix.modules.index_mut((cordinates[0].0, i + FINDER_SIZE)) = *timing_element;
            *self.matrix.modules.index_mut((i + FINDER_SIZE, cordinates[1].1)) = *timing_element;
        }
    }

    fn generate_timing_block(&self) -> Matrix<Module> {
        let mut timing_vector: Vec<Module> = Vec::new();

        let size_timing: usize = self.get_size_timing();

        let mut boolean_timing: bool = true;

        for _ in 0..size_timing {
            timing_vector.push(Module::Function(boolean_timing));

            boolean_timing = !boolean_timing;
        }

        let timing_block: Matrix<Module> = Matrix::from_vec(size_timing, 1, timing_vector);

        return timing_block;
    }

    fn get_timing_cordinates() -> [(usize, usize); 2] {
        return [(6, 8), (8, 6)];
    }

    fn get_size_timing(&self) -> usize {
        return  self.matrix.size - 2 * FINDER_SIZE;
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
    let mut qr_block: QrMatrix = QrMatrix::new(21);
    let mut finder_builder: FinderBuilder = FinderBuilder { matrix: &mut qr_block };

    finder_builder.add_finders();

    let mut timing_builder: TimingBuilder = TimingBuilder { matrix: &mut qr_block };
    timing_builder.add_timing_blocks();

    qr_block.print_matrix();
}