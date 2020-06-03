use crate::types::{Int, Long, Float};

#[derive(Debug, Copy, Clone)]
pub struct Double {
    value: f64
}

impl Double {
    pub fn new(value: f64) -> Self {
        Double {
            value
        }
    }

    pub fn add(&self, other: Double) -> Double {
        let result = self.value + other.value;
        Double::new(result)
    }

    pub fn sub(&self, other: Double) -> Double {
        let result = self.value - other.value;
        Double::new(result)
    }

    pub fn mul(&self, other: Double) -> Double {
        let result = self.value * other.value;
        Double::new(result)
    }

    pub fn div(&self, other: Double) -> Double {
        let result = self.value / other.value;
        Double::new(result)
    }

    pub fn rem(&self, other: Double) -> Double {
        let result = self.value % other.value;
        Double::new(result)
    }

    pub fn neg(&self) -> Double {
        let result = -self.value;
        Double::new(result)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.value as i32)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.value as i64)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.value as f32)
    }

    pub fn eq(&self, other: Double) -> bool {
        self.value == other.value
    }

    pub fn lt(&self, other: Double) -> bool {
        self.value < other.value
    }

    pub fn gt(&self, other: Double) -> bool {
        self.value > other.value
    }

    pub fn is_nan(&self) -> bool {
        self.value.is_nan()
    }
}