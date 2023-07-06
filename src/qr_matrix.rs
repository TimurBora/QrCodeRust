
use crate::module::Module;

use crate::get_height;
use crate::get_width;

use::generic_matrix;
use generic_matrix::Matrix;

use std::ops::Index;
use std::ops::IndexMut;

pub struct QrMatrix {
    size: usize,
    modules: Matrix<Module>,
}

impl QrMatrix {
    pub fn new(size: usize) -> Self {
        QrMatrix { size: size, modules: Matrix::from_vec(size, size, vec![Module::Unknown; size * size]) }
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    pub fn get_modules(&mut self) -> &mut Matrix<Module> {
        return &mut self.modules;
    }

    pub fn set_rect(&mut self, cordinate1: (usize, usize), cordinate2: (usize, usize), matrix_module: &mut Matrix<Module>) {
        let start_width: usize = cordinate1.1;
        let start_height: usize = cordinate1.0;

        let width: usize = get_width(&cordinate1, &cordinate2) - start_width;
        let height: usize = get_height(&cordinate1, &cordinate2) - start_height;

        for i in 0..height {
            for j in 0..width {
                let mut _module: &mut Module = self.modules.index_mut((i + start_height, j + start_width));
                *_module = *matrix_module.index_mut((i, j));
            }
        }
    }

    pub fn set_square(&mut self, size: usize, cordinate: (usize, usize), matrix_module: &mut Matrix<Module>) {
        let start_width: usize = cordinate.1;
        let start_height: usize = cordinate.0;

        for i in 0..start_height - size {
            for j in 0..start_width - size {
                let mut _module: &mut Module = self.modules.index_mut((i, j));
                *_module = *matrix_module.index_mut((i, j));
            }
        }
    }

    pub fn print_matrix(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.modules.index((i, j)) {
                    Module::Unknown => print!("-"),
                    Module::Function(true) => print!("1"),
                    Module::Function(false) => print!("0"),
                    Module::Data(true) => print!("1"),
                    Module::Data(false) => print!("0"),
                    Module::Reserved => print!("0"),
                    _ => print!("0"),
                }
            }
            println!("");
        }
    }
}