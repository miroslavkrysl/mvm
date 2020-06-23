use crate::types::{Int, Long, Float, Double, Reference, ValueCategory, ValueError};
use crate::types::Categorize;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompValue {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}

impl Categorize for CompValue {
    fn category(&self) -> ValueCategory {
        match self {
            CompValue::Int(value) => value.category(),
            CompValue::Long(value) => value.category(),
            CompValue::Float(value) => value.category(),
            CompValue::Double(value) => value.category(),
            CompValue::Reference(value) => value.category(),
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