use crate::types::{Int, Long, Float, Double, ValueError};
use crate::class::{ClassSymRef, FieldSymRef, MethodSymRef};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ConstantValue {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Class(ClassSymRef),
    Field(FieldSymRef),
    Method(MethodSymRef),
}

impl<'a> TryFrom<&'a ConstantValue> for &'a Int {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Int(int) => Ok(int),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a Long {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Long(long) => Ok(long),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a Float {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Float(float) => Ok(float),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a Double {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Double(double) => Ok(double),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a ClassSymRef {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Class(sym_ref) => Ok(sym_ref),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a FieldSymRef {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Field(sym_ref) => Ok(sym_ref),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl<'a> TryFrom<&'a ConstantValue> for &'a MethodSymRef {
    type Error = ValueError;

    fn try_from(value: &'a ConstantValue) -> Result<Self, Self::Error> {
        match &value {
            ConstantValue::Method(sym_ref) => Ok(sym_ref),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}

impl From<Int> for ConstantValue {
    fn from(int: Int) -> Self {
        ConstantValue::Int(int)
    }
}

impl From<Long> for ConstantValue {
    fn from(long: Long) -> Self {
        ConstantValue::Long(long)
    }
}

impl From<Float> for ConstantValue {
    fn from(float: Float) -> Self {
        ConstantValue::Float(float)
    }
}

impl From<Double> for ConstantValue {
    fn from(double: Double) -> Self {
        ConstantValue::Double(double)
    }
}

impl From<ClassSymRef> for ConstantValue {
    fn from(sym_ref: ClassSymRef) -> Self {
        ConstantValue::Class(sym_ref)
    }
}

impl From<FieldSymRef> for ConstantValue {
    fn from(sym_ref: FieldSymRef) -> Self {
        ConstantValue::Field(sym_ref)
    }
}

impl From<MethodSymRef> for ConstantValue {
    fn from(sym_ref: MethodSymRef) -> Self {
        ConstantValue::Method(sym_ref)
    }
}

impl fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantValue::Int(value) => write!(f, "{}", value),
            ConstantValue::Long(value) => write!(f, "{}", value),
            ConstantValue::Float(value) => write!(f, "{}", value),
            ConstantValue::Double(value) => write!(f, "{}", value),
            ConstantValue::Class(value) => write!(f, "{}", value),
            ConstantValue::Field(value) => write!(f, "{}", value),
            ConstantValue::Method(value) => write!(f, "{}", value),
        }
    }
}