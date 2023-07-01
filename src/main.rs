
use std::ops::{IndexMut, Index};

use::generic_matrix;
use generic_matrix::Matrix;

const SCAN_BLOCK: Vec<[i32; 8]> = vec![[1, 1, 1, 1, 1, 1, 1, 0],
                                       [1, 0, 0, 0, 0, 0, 1, 0],
                                       [1, 0, 1, 1, 1, 0, 1, 0],
                                       [1, 0, 1, 1, 1, 0, 1, 0],
                                       [1, 0, 1, 1, 1, 0, 1, 0],
                                       [1, 0, 0, 0, 0, 0, 1, 0],
                                       [1, 1, 1, 1, 1, 1, 1, 0],
                                       [0, 0, 0, 0, 0, 0, 0, 0]];

#[derive(Clone, Copy)]
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
                    Module::Unknown => print!("0"),
                    _ => print!(" "),
                }
            }
            println!("");
        }
    }
}

struct QrBuilder {
    matrix: QrMatrix,   
}

impl QrBuilder {
    fn create_finder() -> Vec<Module> {
        let mut finder_matrix: Matrix<Module> = Matrix::from_vec(7, 7, vec![Module::Function(true); 49]);
        let mut finder_vector: Vec<Module>;

        for i in 0..finder_matrix.row() {
            for j in 0..finder_matrix.column() {
                if i == 1 || i == 5 || j == 1 || j == 5 {
                    let mut element: Module = *finder_matrix.index_mut((i, j));
                    element = Module::Function(false);
                }
                finder_vector.push(*finder_matrix.index((i, j)));
            }
        }

        return finder_vector;
    }

    fn add_finder(&mut self) -> Matrix<Module> {
        let finder_vector: Vec<Module> = QrBuilder::create_finder();
        let cordinate_vec: Vec<(usize, usize)> = vec![(0, 0),
                                                      (0, self.matrix.modules.column() - 7), 
                                                      (self.matrix.modules.row() - 7, 0)];
        
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
    let mut matrix: QrMatrix = QrMatrix::new(7);
    matrix.set_rect((0, 0), (3, 1), Module::Data(true));
    matrix.print_matrix();
}