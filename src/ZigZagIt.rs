

#[derive(Debug, Clone, Copy)]
pub struct ZigZagIt {
    matrix_size: usize,
    horizontaly_next: bool,
    upward: bool,
    pub row_cordinate: usize,
    pub column_cordinate: usize,
    valid: bool,
}

impl ZigZagIt {
    pub fn new(matrix_size: usize) -> Self {
        return ZigZagIt {
            matrix_size: matrix_size,
            horizontaly_next: true,
            upward: true,
            row_cordinate: matrix_size - 1,
            column_cordinate: matrix_size - 1,
            valid: true,
        };
    }

    fn move_cordinate(&mut self) {
        if self.horizontaly_next {
            self.move_horizontaly();
        }
        else {
            self.move_vertical();
        }
    }

    fn move_horizontaly(&mut self) {
        match self.column_cordinate {
            0 => self.valid = false,
            6 => self.column_cordinate -= 2,
            _ => self.column_cordinate -= 1,
        }

        self.horizontaly_next = false;
    }

    fn move_vertical(&mut self) {
        if (self.upward && self.row_cordinate == 0) || (!self.upward && self.row_cordinate == self.matrix_size - 1) {
            self.change_upward();
            self.move_horizontaly();
        }
        else {
            if self.upward { self.row_cordinate -= 1 }
            else if !self.upward { self.row_cordinate += 1 }
            self.column_cordinate += 1;
        }

        self.horizontaly_next = true;
    }

    fn change_upward(&mut self) {
        self.upward = !self.upward;
    }
}

impl Iterator for ZigZagIt {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid { return None; }

        let cordinates: (usize, usize) = (self.row_cordinate, self.column_cordinate);

        self.move_cordinate();

        return Some(cordinates);
    }
}