use crate::types::category::{Categorize, ValueCategory};
use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Reference {
    Null
}

impl Reference {
    pub fn null() -> Self {
        Reference::Null
    }
}

impl Categorize for Reference {
    fn category(&self) -> ValueCategory {
        ValueCategory::Single
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ref")
    }
}