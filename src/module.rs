

#[derive(Clone, Copy, PartialEq)]
pub enum Module {
    Unknown,
    Reserved,
    Data(bool),
    Function(bool),
}

impl Module {

}