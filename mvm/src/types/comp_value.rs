
use std::convert::TryFrom;
use std::fmt;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::float::Float;
use crate::types::double::Double;
use crate::types::reference::Reference;
use crate::types::category::{ValueCategory, Categorize};
use crate::types::error::ValueError;


#[derive(Debug, Clone, PartialEq)]
pub enum CompValue {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}

impl CompValue {
    pub fn category(&self) -> ValueCategory {
        match self {
            CompValue::Int(value) => Int::category(),
            CompValue::Long(value) => Long::category(),
            CompValue::Float(value) => Float::category(),
            CompValue::Double(value) => Double::category(),
            CompValue::Reference(value) => Reference::category(),
        }
    }
}


impl TryFrom<CompValue> for Int {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Int(int) => Ok(int),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<CompValue> for Long {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Long(long) => Ok(long),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<CompValue> for Float {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Float(float) => Ok(float),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<CompValue> for Double {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Double(double) => Ok(double),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<CompValue> for Reference {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Reference(reference) => Ok(reference),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl From<Int> for CompValue {
    fn from(int: Int) -> Self {
        CompValue::Int(int)
    }
}


impl From<Long> for CompValue {
    fn from(long: Long) -> Self {
        CompValue::Long(long)
    }
}


impl From<Float> for CompValue {
    fn from(float: Float) -> Self {
        CompValue::Float(float)
    }
}


impl From<Double> for CompValue {
    fn from(double: Double) -> Self {
        CompValue::Double(double)
    }
}


impl From<Reference> for CompValue {
    fn from(reference: Reference) -> Self {
        CompValue::Reference(reference)
    }
}

impl fmt::Display for CompValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompValue::Int(value) => write!(f, "{}", value),
            CompValue::Long(value) => write!(f, "{}", value),
            CompValue::Float(value) => write!(f, "{}", value),
            CompValue::Double(value) => write!(f, "{}", value),
            CompValue::Reference(value) => write!(f, "{}", value),
        }
    }
}