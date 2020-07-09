use std::convert::TryFrom;
use std::fmt;
use crate::types::byte::Byte;
use crate::types::short::Short;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::float::Float;
use crate::types::double::Double;
use crate::types::reference::Reference;
use crate::class::descriptor::TypeDescriptor;
use crate::types::category::Describe;
use crate::types::error::ValueError;


#[derive(Debug, Clone)]
pub enum JvmValue {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}

impl JvmValue {
    pub fn descriptor(&self) -> TypeDescriptor {
        match self {
            JvmValue::Byte(value) => Byte::descriptor(),
            JvmValue::Short(value) => Short::descriptor(),
            JvmValue::Int(value) => Int::descriptor(),
            JvmValue::Long(value) => Long::descriptor(),
            JvmValue::Float(value) => Float::descriptor(),
            JvmValue::Double(value) => Double::descriptor(),
            JvmValue::Reference(value) => Reference::descriptor(),
        }
    }
}

impl TryFrom<JvmValue> for Int {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Int(int) => Ok(int),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Long {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Long(long) => Ok(long),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Float {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Float(float) => Ok(float),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Double {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Double(double) => Ok(double),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Byte {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Byte(byte) => Ok(byte),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Short {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Short(short) => Ok(short),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Reference {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Reference(reference) => Ok(reference),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl From<Int> for JvmValue {
    fn from(int: Int) -> Self {
        JvmValue::Int(int)
    }
}


impl From<Long> for JvmValue {
    fn from(long: Long) -> Self {
        JvmValue::Long(long)
    }
}


impl From<Float> for JvmValue {
    fn from(float: Float) -> Self {
        JvmValue::Float(float)
    }
}


impl From<Double> for JvmValue {
    fn from(double: Double) -> Self {
        JvmValue::Double(double)
    }
}


impl From<Reference> for JvmValue {
    fn from(reference: Reference) -> Self {
        JvmValue::Reference(reference)
    }
}


impl From<Byte> for JvmValue {
    fn from(byte: Byte) -> Self {
        JvmValue::Byte(byte)
    }
}


impl From<Short> for JvmValue {
    fn from(short: Short) -> Self {
        JvmValue::Short(short)
    }
}


impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JvmValue::Int(value) => write!(f, "{}", value),
            JvmValue::Long(value) => write!(f, "{}", value),
            JvmValue::Float(value) => write!(f, "{}", value),
            JvmValue::Double(value) => write!(f, "{}", value),
            JvmValue::Reference(value) => write!(f, "{}", value),
            JvmValue::Byte(value) => write!(f, "{}", value),
            JvmValue::Short(value) => write!(f, "{}", value),
        }
    }
}