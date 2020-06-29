use std::fmt;
use crate::types::Describe;
use crate::class::{ValueDescriptor, SimpleValueDescriptor};

#[derive(Debug, Copy, Clone)]
pub struct Char(u16);

impl Char {
    pub fn new(value: u16) -> Self {
        Char(value)
    }
}

impl From<Char> for u16 {
    fn from(char: Char) -> Self {
        char.0
    }
}

impl Default for Char {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Display for Char {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Describe for Char {
    fn descriptor(&self) -> ValueDescriptor {
        ValueDescriptor::Simple(SimpleValueDescriptor::Char)
    }
}