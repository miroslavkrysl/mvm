use std::fmt;

use crate::vm::types::double::Double;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;


#[derive(Debug, Clone, PartialEq)]
pub struct Float(f32);


impl Float {
    pub fn new(value: f32) -> Self {
        Float(value)
    }

    pub fn add(&self, other: &Float) -> Float {
        let result = self.0 + other.0;
        Float::new(result)
    }

    pub fn sub(&self, other: &Float) -> Float {
        let result = self.0 - other.0;
        Float::new(result)
    }

    pub fn mul(&self, other: &Float) -> Float {
        let result = self.0 * other.0;
        Float::new(result)
    }

    pub fn div(&self, other: &Float) -> Float {
        let result = self.0 / other.0;
        Float::new(result)
    }

    pub fn rem(&self, other: &Float) -> Float {
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

    pub fn cmpl(&self, other: &Float) -> Int {
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

    pub fn cmpg(&self, other: &Float) -> Int {
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
}


impl From<Float> for f32 {
    fn from(value: Float) -> Self {
        value.0
    }
}


impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Float::new(value)
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
