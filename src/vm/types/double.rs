use std::fmt;

use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;


#[derive(Debug, Clone, PartialEq)]
pub struct Double(f64);


impl Double {
    pub fn new(value: f64) -> Self {
        Double(value)
    }

    pub fn add(&self, other: &Double) -> Double {
        let result = self.0 + other.0;
        Double::new(result)
    }

    pub fn sub(&self, other: &Double) -> Double {
        let result = self.0 - other.0;
        Double::new(result)
    }

    pub fn mul(&self, other: &Double) -> Double {
        let result = self.0 * other.0;
        Double::new(result)
    }

    pub fn div(&self, other: &Double) -> Double {
        let result = self.0 / other.0;
        Double::new(result)
    }

    pub fn rem(&self, other: &Double) -> Double {
        let result = self.0 % other.0;
        Double::new(result)
    }

    pub fn neg(&self) -> Double {
        let result = -self.0;
        Double::new(result)
    }

    pub fn cmpl(&self, other: &Double) -> Int {
        if self.0 > other.0 {
            Int::new(1)
        } else if self.0 == other.0 {
            Int::new(0)
        } else if self.0 < other.0 {
            Int::new(-1)
        } else {
            Int::new(1)
        }
    }

    pub fn cmpg(&self, other: &Double) -> Int {
        if self.0 > other.0 {
            Int::new(1)
        } else if self.0 == other.0 {
            Int::new(0)
        } else if self.0 < other.0 {
            Int::new(-1)
        } else {
            Int::new(-1)
        }
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
}


impl From<Double> for f64 {
    fn from(value: Double) -> Self {
        value.0
    }
}


impl From<f64> for Double {
    fn from(value: f64) -> Self {
        Double::new(value)
    }
}


impl Default for Double {
    fn default() -> Self {
        Self::new(0.0)
    }
}


impl fmt::Display for Double {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
