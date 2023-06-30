

struct Module {
    value: bool,
    is_define: bool,
    cordinate: (i32, i32),
}

struct Block {
    width: i32,
    height: i32,
    matrix: Vec<Vec<Module>>,
}

impl Block{
    fn new(width: i32, height: i32, value: bool, is_define: bool) -> Self {
        let mut matrix: Vec<Vec<Module>> = Vec::new();

        for i in 0..height {
            let mut row: Vec<Module> = Vec::new();
            for j in 0..width {
                let module: Module = Module { value: value, is_define: is_define, cordinate: (i, j) };
                row.push(module);
            }

            matrix.push(row);
        }

        Block { width: width, height: height, matrix: matrix }
    }
}


struct QrCode {
    matrix: Vec<Vec<bool>>,
}

fn main() {
    let block: Block = Block::new(21, 21, false, true);

    for row in block.matrix.iter()
    {
        for element in row
        {
            print!("{}", if element.value { "1" } else { "0" });
        }
        println!();
    }

}