use crate::types::{Long, Float, Double, Categorize, ValueCategory, DivisionByZero, Describe, Short};
use std::fmt;
use crate::class::{ValueDescriptor, SimpleValueDescriptor};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Int(i32);

impl Int {
    pub fn new(value: i32) -> Self {
        Int(value)
    }

    pub fn add(&self, other: &Int) -> Int {
        let result = self.0.wrapping_add(other.0);
        Int::new(result)
    }

    pub fn sub(&self, other: &Int) -> Int {
        let result = self.0.wrapping_sub(other.0);
        Int::new(result)
    }

    pub fn mul(&self, other: &Int) -> Int {
        let result = self.0.wrapping_mul(other.0);
        Int::new(result)
    }

    pub fn div(&self, other: &Int) -> Result<Int, DivisionByZero> {
        if other.0 == 0 {
            return Err(DivisionByZero);
        }
        let result = self.0.wrapping_div(other.0);
        Ok(Int::new(result))
    }

    pub fn rem(&self, other: &Int) -> Int {
        let result = self.0.wrapping_rem(other.0);
        Int::new(result)
    }

    pub fn neg(&self) -> Int {
        let result = self.0.wrapping_neg();
        Int::new(result)
    }

    pub fn shl(&self, shift: Int) -> Int {
        let result = self.0.wrapping_shl(shift.0 as u32);
        Int::new(result)
    }

    pub fn shr(&self, shift: Int) -> Int {
        let result = self.0.wrapping_shr(shift.0 as u32);
        Int::new(result)
    }

    pub fn ushl(&self, shift: Int) -> Int {
        let result = (self.0 as u32).wrapping_shl(shift.0 as u32);
        Int::new(result as i32)
    }

    pub fn ushr(&self, shift: Int) -> Int {
        let result = (self.0 as u32).wrapping_shr(shift.0 as u32);
        Int::new(result as i32)
    }

    pub fn and(&self, other: &Int) -> Int {
        let result = self.0 & other.0;
        Int::new(result)
    }

    pub fn or(&self, other: &Int) -> Int {
        let result = self.0 | other.0;
        Int::new(result)
    }

    pub fn xor(&self, other: &Int) -> Int {
        let result = self.0 ^ other.0;
        Int::new(result)
    }

    pub fn inc(&self, value: &Int) -> Int {
        self.add(&value)
    }

    pub fn to_long(&self) -> Long {
        Long::new(self.0 as i64)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.0 as f32)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.0 as f64)
    }

    pub fn to_byte(&self) -> Int {
        Int::new(self.0 as i8 as i32)
    }

    pub fn to_char(&self) -> Int {
        Int::new(self.0 as u16 as i32)
    }

    pub fn to_short(&self) -> Int {
        Int::new(self.0 as i16 as i32)
    }

    pub fn eq(&self, other: &Int) -> bool {
        self.0 == other.0
    }

    pub fn lt(&self, other: &Int) -> bool {
        self.0 < other.0
    }

    pub fn le(&self, other: &Int) -> bool {
        self.0 <= other.0
    }

    pub fn gt(&self, other: &Int) -> bool {
        self.0 > other.0
    }

    pub fn ge(&self, other: &Int) -> bool {
        self.0 >= other.0
    }
}

impl From<Int> for i32 {
    fn from(int: Int) -> Self {
        int.0
    }
}

impl Categorize for Int {
    fn category(&self) -> ValueCategory {
        ValueCategory::Single
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Int {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Describe for Int {
    fn descriptor(&self) -> ValueDescriptor {
        ValueDescriptor::Simple(SimpleValueDescriptor::Int)
    }
}