use crate::types::error::DivisionByZero;
use crate::types::{Long, Float, Double};

#[derive(Debug, Copy, Clone)]
pub struct Int {
    value: i32
}

impl Int {
    pub fn new(value: i32) -> Self {
        Int {
            value
        }
    }

    pub fn add(&self, other: Int) -> Int {
        let result = self.value.wrapping_add(other.value);
        Int::new(result)
    }

    pub fn sub(&self, other: Int) -> Int {
        let result = self.value.wrapping_sub(other.value);
        Int::new(result)
    }

    pub fn mul(&self, other: Int) -> Int {
        let result = self.value.wrapping_mul(other.value);
        Int::new(result)
    }

    pub fn div(&self, other: Int) -> Result<Int, DivisionByZero> {
        if other.value == 0 {
            return Err(DivisionByZero);
        }
        let result = self.value.wrapping_div(other.value);
        Ok(Int::new(result))
    }

    pub fn rem(&self, other: Int) -> Int {
        let result = self.value.wrapping_rem(other.value);
        Int::new(result)
    }

    pub fn neg(&self) -> Int {
        let result = self.value.wrapping_neg();
        Int::new(result)
    }

    pub fn shl(&self, shift: Int) -> Int {
        let result = self.value.wrapping_shl(shift.value as u32);
        Int::new(result)
    }

    pub fn shr(&self, shift: Int) -> Int {
        let result = self.value.wrapping_shr(shift.value as u32);
        Int::new(result)
    }

    pub fn ushl(&self, shift: Int) -> Int {
        let result = (self.value as u32).wrapping_shl(shift.value as u32);
        Int::new(result as i32)
    }

    pub fn ushr(&self, shift: Int) -> Int {
        let result = (self.value as u32).wrapping_shr(shift.value as u32);
        Int::new(result as i32)
    }

    pub fn and(&self, other: Int) -> Int {
        let result = self.value & other.value;
        Int::new(result)
    }

    pub fn or(&self, other: Int) -> Int {
        let result = self.value | other.value;
        Int::new(result)
    }

    pub fn xor(&self, other: Int) -> Int {
        let result = self.value ^ other.value;
        Int::new(result)
    }

    pub fn inc(&self, value: Int) -> Int {
        self.add(value)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.value as i64)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.value as f32)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.value as f64)
    }

    pub fn to_byte(&self) -> Int {
        Int::new(self.value as i8 as i32)
    }

    pub fn to_char(&self) -> Int {
        Int::new(self.value as u16 as i32)
    }

    pub fn to_short(&self) -> Int {
        Int::new(self.value as i16 as i32)
    }

    pub fn eq(&self, other: Int) -> bool {
        self.value == other.value
    }

    pub fn lt(&self, other: Int) -> bool {
        self.value < other.value
    }

    pub fn le(&self, other: Int) -> bool {
        self.value <= other.value
    }

    pub fn gt(&self, other: Int) -> bool {
        self.value > other.value
    }

    pub fn ge(&self, other: Int) -> bool {
        self.value >= other.value
    }
}

impl From<Int> for i32 {
    fn from(int: Int) -> Self {
        int.value
    }
}
