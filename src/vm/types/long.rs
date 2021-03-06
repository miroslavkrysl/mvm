use std::fmt;

use crate::vm::types::double::Double;
use crate::vm::types::error::ValueError;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;


#[derive(Debug, Clone, PartialEq)]
pub struct Long(i64);


impl Long {
    pub fn new(value: i64) -> Self {
        Long(value)
    }

    pub fn add(&self, other: &Long) -> Long {
        let result = self.0.wrapping_add(other.0);
        Long::new(result)
    }

    pub fn sub(&self, other: &Long) -> Long {
        let result = self.0.wrapping_sub(other.0);
        Long::new(result)
    }

    pub fn mul(&self, other: &Long) -> Long {
        let result = self.0.wrapping_mul(other.0);
        Long::new(result)
    }

    pub fn div(&self, other: &Long) -> Result<Long, ValueError> {
        if other.0 == 0 {
            return Err(ValueError::DivisionByZero);
        }
        let result = self.0.wrapping_div(other.0);
        Ok(Long::new(result))
    }

    pub fn rem(&self, other: &Long) -> Long {
        let result = self.0.wrapping_rem(other.0);
        Long::new(result)
    }

    pub fn neg(&self) -> Long {
        let result = self.0.wrapping_neg();
        Long::new(result)
    }

    pub fn shl(&self, shift: &Int) -> Long {
        let shift: i32 = shift.clone().into();
        let result = self.0.wrapping_shl(shift as u32);
        Long::new(result)
    }

    pub fn shr(&self, shift: &Int) -> Long {
        let shift: i32 = shift.clone().into();
        let result = self.0.wrapping_shr(shift as u32);
        Long::new(result)
    }

    pub fn ushr(&self, shift: &Int) -> Long {
        let shift: i32 = shift.clone().into();
        let result = (self.0 as u64).wrapping_shr(shift as u32);
        Long::new(result as i64)
    }

    pub fn and(&self, other: &Long) -> Long {
        let result = self.0 & other.0;
        Long::new(result)
    }

    pub fn or(&self, other: &Long) -> Long {
        let result = self.0 | other.0;
        Long::new(result)
    }

    pub fn xor(&self, other: &Long) -> Long {
        let result = self.0 ^ other.0;
        Long::new(result)
    }

    pub fn cmp(&self, other: &Long) -> Int {
        if self.0 > other.0 {
            Int::new(1)
        } else if self.0 == other.0 {
            Int::new(0)
        } else {
            Int::new(-1)
        }
    }

    pub fn to_int(&self) -> Int {
        Int::new(self.0 as i32)
    }

    pub fn to_float(&self) -> Float {
        Float::new(self.0 as f32)
    }

    pub fn to_double(&self) -> Double {
        Double::new(self.0 as f64)
    }
}


impl From<Long> for i64 {
    fn from(value: Long) -> Self {
        value.0
    }
}


impl From<i64> for Long {
    fn from(value: i64) -> Self {
        Long::new(value)
    }
}


impl fmt::Display for Long {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl Default for Long {
    fn default() -> Self {
        Self::new(0)
    }
}
