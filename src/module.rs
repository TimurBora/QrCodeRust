#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Module {
    Unknown,
    //Reserved,
    Data(bool),
    Function(bool),
}

impl Module {
    pub fn set_module(&mut self, new_module: Module) {
        *self = new_module;
    }

    pub fn is_fun(&self) -> bool {
        match self {
            Module::Function(_) => return true,
            _ => return false,
        }
    }

    pub fn flip_module(&mut self) {
        match self {
            Self::Data(true) => *self = Self::Data(false),
            Self::Data(false) => *self = Self::Data(true),
            _ => (),
        }
    }
}
