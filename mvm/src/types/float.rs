use crate::types::{Int, Long, Double};

#[derive(Debug, Copy, Clone)]
pub struct Float {
    value: f32
}

impl Float {
    pub fn new(value: f32) -> Self {
        Float {
            value
        }
    }

    pub fn add(&self, other: Float) -> Float {
        let result = self.value + other.value;
        Float::new(result)
    }

    pub fn sub(&self, other: Float) -> Float {
        let result = self.value - other.value;
        Float::new(result)
    }

    pub fn mul(&self, other: Float) -> Float {
        let result = self.value * other.value;
        Float::new(result)
    }

    pub fn div(&self, other: Float) -> Float {
        let result = self.value / other.value;
        Float::new(result)
    }

    pub fn rem(&self, other: Float) -> Float {
        let result = self.value % other.value;
        Float::new(result)
    }

    pub fn neg(&self) -> Float {
        let result = -self.value;
        Float::new(result)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.value as i32)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.value as i64)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.value as f64)
    }

    pub fn eq(&self, other: Float) -> bool {
        self.value == other.value
    }

    pub fn lt(&self, other: Float) -> bool {
        self.value < other.value
    }

    pub fn gt(&self, other: Float) -> bool {
        self.value > other.value
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }
}