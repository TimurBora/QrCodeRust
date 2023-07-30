

struct Version {
    version: usize,
}

impl Version {
    pub fn new(version: usize) -> Self {
        assert!(version >= 1 || version <= 40);

        return Self { version: version };
    }

    pub fn index(&self) -> usize {
        return self.version - 1;
    }

    pub fn get_matrix_size(&self) -> usize {
        return self.index() * 4 + 21;
    }
}