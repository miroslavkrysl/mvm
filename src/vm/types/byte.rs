use std::fmt;

use crate::vm::types::int::Int;


#[derive(Debug, Clone, PartialEq)]
pub struct Byte(i8);


impl Byte {
    pub fn new(value: i8) -> Self {
        Byte(value)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.0 as i32)
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

