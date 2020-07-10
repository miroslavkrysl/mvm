use std::convert::TryFrom;
use std::fmt;
use crate::types::byte::Byte;
use crate::types::short::Short;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::float::Float;
use crate::types::double::Double;
use crate::types::reference::Reference;
use crate::class::descriptor::TypeDesc;
use crate::types::category::Describe;
use crate::types::error::ValueError;


#[derive(Debug, Clone)]
pub enum Value {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}

impl Value {
    pub fn descriptor(&self) -> TypeDesc {
        match self {
            Value::Byte(value) => Byte::descriptor(),
            Value::Short(value) => Short::descriptor(),
            Value::Int(value) => Int::descriptor(),
            Value::Long(value) => Long::descriptor(),
            Value::Float(value) => Float::descriptor(),
            Value::Double(value) => Double::descriptor(),
            Value::Reference(value) => Reference::descriptor(),
        }
    }

    pub fn is_byte(&self) -> bool {
        if let Value::Byte(_) = self {true} else {false}
    }

    pub fn is_short(&self) -> bool {
        if let Value::Short(_) = self {true} else {false}
    }

    pub fn is_int(&self) -> bool {
        if let Value::Int(_) = self {true} else {false}
    }

    pub fn is_long(&self) -> bool {
        if let Value::Long(_) = self {true} else {false}
    }

    pub fn is_float(&self) -> bool {
        if let Value::Float(_) = self {true} else {false}
    }

    pub fn is_double(&self) -> bool {
        if let Value::Double(_) = self {true} else {false}
    }

    pub fn is_reference(&self) -> bool {
        if let Value::Reference(_) = self {true} else {false}
    }
}

impl TryFrom<Value> for Int {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(int) => Ok(int),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Long {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Long(long) => Ok(long),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Float {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(float) => Ok(float),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Double {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Double(double) => Ok(double),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Byte {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Byte(byte) => Ok(byte),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Short {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Short(short) => Ok(short),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<Value> for Reference {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Reference(reference) => Ok(reference),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl From<Int> for Value {
    fn from(int: Int) -> Self {
        Value::Int(int)
    }
}


impl From<Long> for Value {
    fn from(long: Long) -> Self {
        Value::Long(long)
    }
}


impl From<Float> for Value {
    fn from(float: Float) -> Self {
        Value::Float(float)
    }
}


impl From<Double> for Value {
    fn from(double: Double) -> Self {
        Value::Double(double)
    }
}


impl From<Reference> for Value {
    fn from(reference: Reference) -> Self {
        Value::Reference(reference)
    }
}


impl From<Byte> for Value {
    fn from(byte: Byte) -> Self {
        Value::Byte(byte)
    }
}


impl From<Short> for Value {
    fn from(short: Short) -> Self {
        Value::Short(short)
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Long(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Double(value) => write!(f, "{}", value),
            Value::Reference(value) => write!(f, "{}", value),
            Value::Byte(value) => write!(f, "{}", value),
            Value::Short(value) => write!(f, "{}", value),
        }
    }
}