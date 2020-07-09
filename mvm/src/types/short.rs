use std::fmt;
use crate::types::category::{Describe, ValueCategory, Categorize};
use crate::class::descriptor::{TypeDescriptor, SimpleDescriptor};


#[derive(Debug, Copy, Clone)]
pub struct Short(i16);

impl Short {
    pub fn new(value: i16) -> Self {
        Short(value)
    }
}

impl From<Short> for i16 {
    fn from(short: Short) -> Self {
        short.0
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

impl Describe for Short {
    fn descriptor() -> TypeDescriptor {
        TypeDescriptor::Simple(SimpleDescriptor::Short)
    }
}

impl Categorize for Short {
    fn category() -> ValueCategory {
        ValueCategory::Single
    }
}