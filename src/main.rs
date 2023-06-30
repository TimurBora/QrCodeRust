

const SIZE_SCAN: i32 = 7;

struct Module {
    value: bool,
    is_define: bool,
    cordinate: (i32, i32),
}

impl Module {
    fn set_module(&mut self, value: bool) {
        self.value = value;
    }
}

struct Block {
    width: i32,
    height: i32,
    matrix: Vec<Vec<Module>>,
    value: bool,
}

impl Block {
    fn new(width: i32, height: i32, value: bool, is_define: bool) -> Self {
        let mut matrix: Vec<Vec<Module>> = Vec::new();

        for i in 0..height {
            let mut row: Vec<Module> = Vec::new();
            for j in 0..width {
                let mut module: Module = Module { value, is_define, cordinate: (i, j) };
                row.push(module);
            }

            matrix.push(row);
        }

        let mut block = Block { width: width, height: height, value: value, matrix: matrix };

        return block;
    }

    fn print_block(&self) {
        for row in self.matrix.iter() {
            for element in row
            {
                print!("{}", if element.value { "1" } else { "0" });
            }
            println!();
        }
    }
}

struct ScanBlock {
    block: Block,
}

impl ScanBlock {
    fn new() -> Self {
        let block: Block = Block::new(SIZE_SCAN, SIZE_SCAN, true, true);

        let scan_block: ScanBlock = ScanBlock { block: block };

        return scan_block;
    }

    fn shaping(&self) {
        let mut scan_matrix: &Vec<Vec<Module>> = &self.block.matrix;
        
        for row in scan_matrix.iter() {
            for element in row {
                if element.cordinate.0 == 1 || element.cordinate.0 == 5 || element.cordinate.1 == 1 || element.cordinate.1 == 5 {
                    element.set_module(false);
                }
            }
        }
    }
}


struct QrCode {
    main_matrix: Block,
    main_cordinate: Vec<Vec<(i32, i32)>>,
}

fn main() {

    let scan_block1: ScanBlock = ScanBlock::new();
    scan_block1.block.print_block();


}