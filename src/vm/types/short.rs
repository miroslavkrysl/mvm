use std::fmt;

use crate::vm::types::int::Int;


#[derive(Debug, Clone)]
pub struct Short(i16);


impl Short {
    pub fn new(value: i16) -> Self {
        Short(value)
    }

    pub fn to_int(&elf) -> Int {
        Int::new(self.0 as i32)
    }
}


impl From<Short> for i16 {
    fn from(value: Short) -> Self {
        value.0
    }
}


impl From<i16> for Short {
    fn from(value: i16) -> Self {
        Short::new(value)
    }
}


impl Default for Short {
    fn default() -> Self {
        Self::new(0)
    }
}


impl fmt::Display for Short {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
