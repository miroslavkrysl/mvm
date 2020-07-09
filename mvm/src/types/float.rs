
use std::fmt;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::double::Double;
use crate::types::category::{Categorize, ValueCategory, Describe};
use crate::class::descriptor::{TypeDescriptor, SimpleDescriptor};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float(f32);

impl Float {
    pub fn new(value: f32) -> Self {
        Float(value)
    }

    pub fn add(&self, other: Float) -> Float {
        let result = self.0 + other.0;
        Float::new(result)
    }

    pub fn sub(&self, other: Float) -> Float {
        let result = self.0 - other.0;
        Float::new(result)
    }

    pub fn mul(&self, other: Float) -> Float {
        let result = self.0 * other.0;
        Float::new(result)
    }

    pub fn div(&self, other: Float) -> Float {
        let result = self.0 / other.0;
        Float::new(result)
    }

    pub fn rem(&self, other: Float) -> Float {
        let result = self.0 % other.0;
        Float::new(result)
    }

    pub fn neg(&self) -> Float {
        let result = -self.0;
        Float::new(result)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.0 as i32)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.0 as i64)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.0 as f64)
    }

    pub fn eq(&self, other: Float) -> bool {
        self.0 == other.0
    }

    pub fn lt(&self, other: Float) -> bool {
        self.0 < other.0
    }

    pub fn gt(&self, other: Float) -> bool {
        self.0 > other.0
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

impl Categorize for Float {
    fn category() -> ValueCategory {
        ValueCategory::Single
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Float {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl Describe for Float {
    fn descriptor() -> TypeDescriptor {
        TypeDescriptor::Simple(SimpleDescriptor::Float)
    }
}