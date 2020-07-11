use std::fmt;


#[derive(Debug, Copy, Clone)]
pub struct Byte(i8);


impl Byte {
    pub fn new(value: i8) -> Self {
        Byte(value)
    }
}


impl From<Byte> for i8 {
    fn from(byte: Byte) -> Self {
        byte.0
    }
}


impl Default for Byte {
    fn default() -> Self {
        Self::new(0)
    }
}


impl fmt::Display for Byte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
