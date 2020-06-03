use crate::types::error::DivisionByZero;
use crate::types::{Int, Float, Double};

#[derive(Debug, Copy, Clone)]
pub struct Long {
    value: i64
}

impl Long {
    pub fn new(value: i64) -> Self {
        Long {
            value
        }
    }

    pub fn add(&self, other: Long) -> Long {
        let result = self.value.wrapping_add(other.value);
        Long::new(result)
    }

    pub fn sub(&self, other: Long) -> Long {
        let result = self.value.wrapping_sub(other.value);
        Long::new(result)
    }

    pub fn mul(&self, other: Long) -> Long {
        let result = self.value.wrapping_mul(other.value);
        Long::new(result)
    }

    pub fn div(&self, other: Long) -> Result<Long, DivisionByZero> {
        if other.value == 0 {
            return Err(DivisionByZero);
        }
        let result = self.value.wrapping_div(other.value);
        Ok(Long::new(result))
    }

    pub fn rem(&self, other: Long) -> Long {
        let result = self.value.wrapping_rem(other.value);
        Long::new(result)
    }

    pub fn neg(&self) -> Long {
        let result = self.value.wrapping_neg();
        Long::new(result)
    }

    pub fn shl(&self, shift: Int) -> Long {
        let shift: i32 = shift.into();
        let result = self.value.wrapping_shl(shift as u32);
        Long::new(result)
    }

    pub fn shr(&self, shift: Int) -> Long {
        let shift: i32 = shift.into();
        let result = self.value.wrapping_shr(shift as u32);
        Long::new(result)
    }

    pub fn ushl(&self, shift: Int) -> Long {
        let shift: i32 = shift.into();
        let result = (self.value as u64).wrapping_shl(shift as u32);
        Long::new(result as i64)
    }

    pub fn ushr(&self, shift: Int) -> Long {
        let shift: i32 = shift.into();
        let result = (self.value as u64).wrapping_shr(shift as u32);
        Long::new(result as i64)
    }

    pub fn and(&self, other: Long) -> Long {
        let result = self.value & other.value;
        Long::new(result)
    }

    pub fn or(&self, other: Long) -> Long {
        let result = self.value | other.value;
        Long::new(result)
    }

    pub fn xor(&self, other: Long) -> Long {
        let result = self.value ^ other.value;
        Long::new(result)
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.value as i32)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.value as f32)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.value as f64)
    }

    pub fn eq(&self, other: Long) -> bool {
        self.value == other.value
    }

    pub fn lt(&self, other: Long) -> bool {
        self.value < other.value
    }

    pub fn gt(&self, other: Long) -> bool {
        self.value > other.value
    }
}

impl From<Long> for i64 {
    fn from(int: Long) -> Self {
        int.value
    }
}
