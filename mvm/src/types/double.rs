use std::fmt;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::float::Float;
use crate::types::category::{Categorize, ValueCategory, Describe};
use crate::class::descriptor::{TypeDescriptor, SimpleDescriptor};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Double(f64);

impl Double {
    pub fn new(value: f64) -> Self {
        Double(value)
    }

    pub fn add(&self, other: Double) -> Double {
        let result = self.0 + other.0;
        Double::new(result)
    }

    pub fn sub(&self, other: Double) -> Double {
        let result = self.0 - other.0;
        Double::new(result)
    }

    pub fn mul(&self, other: Double) -> Double {
        let result = self.0 * other.0;
        Double::new(result)
    }

    pub fn div(&self, other: Double) -> Double {
        let result = self.0 / other.0;
        Double::new(result)
    }

    pub fn rem(&self, other: Double) -> Double {
        let result = self.0 % other.0;
        Double::new(result)
    }

    pub fn neg(&self) -> Double {
        let result = -self.0;
        Double::new(result)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.0 as i32)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.0 as i64)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.0 as f32)
    }

    pub fn eq(&self, other: Double) -> bool {
        self.0 == other.0
    }

    pub fn lt(&self, other: Double) -> bool {
        self.0 < other.0
    }

    pub fn gt(&self, other: Double) -> bool {
        self.0 > other.0
    }

    pub fn is_nan(&self) -> bool {
        self.0.is_nan()
    }
}

impl Categorize for Double {
    fn category() -> ValueCategory {
        ValueCategory::Double
    }
}

impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Double {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl Describe for Double {
    fn descriptor() -> TypeDescriptor {
        TypeDescriptor::Simple(SimpleDescriptor::Double)
    }
}