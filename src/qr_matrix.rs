use crate::module::Module;

use ::generic_matrix;
use generic_matrix::Matrix;

use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub struct QrMatrix {
    size: usize,
    modules: Matrix<Module>,
}

impl QrMatrix {
    pub fn new(size: usize) -> Self {
        let to_matrix_vec: Vec<Module> = vec![Module::Unknown; size * size];

        let modules: Matrix<Module> = Matrix::from_vec(size, size, to_matrix_vec);
        QrMatrix { size, modules }
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    pub fn get_modules(&mut self) -> &mut Matrix<Module> {
        return &mut self.modules;
    }

    pub fn get_module(&self, cordinate: (usize, usize)) -> &Module {
        return self.modules.index(cordinate);
    }

    pub fn get_mut_module(&mut self, cordinate: (usize, usize)) -> &mut Module {
        return self.modules.index_mut(cordinate);
    }

    pub fn set_square(
        &mut self,
        size: usize,
        cordinate: (usize, usize),
        matrix_module: &mut Matrix<Module>,
    ) {
        let start_width: usize = cordinate.1;
        let start_height: usize = cordinate.0;

        for i in 0..size {
            for j in 0..size {
                let new_module = matrix_module.index_mut((i, j));
                self.set_module((i + start_height, j + start_width), *new_module);
            }
        }
    }

    pub fn set_module(&mut self, cordinate: (usize, usize), new_module: Module) {
        let mut _module: &mut Module = self.get_mut_module(cordinate);
        _module.set_module(new_module);
    }

    pub fn print_matrix(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                match self.get_module((i, j)) {
                    Module::Unknown => print!("■■"),
                    Module::Function(false) => print!("■■"), //■
                    Module::Function(true) => print!("  "),  //█
                    Module::Data(true) => print!("  "),      //□
                    Module::Data(false) => print!("■■"),
                }
            }
            println!("");
        }
    }
}
