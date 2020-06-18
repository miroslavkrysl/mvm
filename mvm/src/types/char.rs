#[derive(Debug, Copy, Clone)]
pub struct Char {
    value: u16
}

impl Char {
    pub fn new(value: u16) -> Self {
        Char {
            value
        }
    }
}

impl From<Char> for u16 {
    fn from(char: Char) -> Self {
        char.value
    }
}
