use std::fmt;
use crate::types::Describe;
use crate::class::{ValueDescriptor, SimpleValueDescriptor};

#[derive(Debug, Copy, Clone)]
pub struct Boolean(bool);

impl Boolean {
    pub fn new(value: bool) -> Self {
        Boolean(value)
    }
}

impl From<Boolean> for bool {
    fn from(boolean: Boolean) -> Self {
        boolean.0
    }
}

impl Default for Boolean {
    fn default() -> Self {
        Self::new(false)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Describe for Boolean {
    fn descriptor(&self) -> ValueDescriptor {
        ValueDescriptor::Simple(SimpleValueDescriptor::Boolean)
    }
}