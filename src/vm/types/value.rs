use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;

use crate::vm::types::double::Double;
use crate::vm::types::error::ValueError;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;
use crate::vm::class::name::ClassName;


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ValueCategory {
    Single,
    Double,
}


impl ValueCategory {
    pub fn size(&self) -> usize {
        match self {
            ValueCategory::Single => 1,
            ValueCategory::Double => 2,
        }
    }
}


#[derive(Debug, Clone)]
pub enum ValueType {
    Int,
    Long,
    Float,
    Double,
    Null,
    Reference(ClassName),
    AnyReference,
    Void,
}


impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Int => write!(f, "int"),
            ValueType::Long => write!(f, "long"),
            ValueType::Float => write!(f, "float"),
            ValueType::Double => write!(f, "double"),
            ValueType::Null => write!(f, "null"),
            ValueType::Reference(class_name) => write!(f, "{}", class_name),
            ValueType::AnyReference => write!(f, "reference"),
            ValueType::Void => write!(f, "void"),
        }
    }
}


impl ValueType {
    pub fn category(&self) -> ValueCategory {
        match self {
            ValueType::Int => ValueCategory::Single,
            ValueType::Long => ValueCategory::Double,
            ValueType::Float => ValueCategory::Single,
            ValueType::Double => ValueCategory::Double,
            ValueType::Null => ValueCategory::Single,
            ValueType::Reference(_) => ValueCategory::Single,
            ValueType::AnyReference => ValueCategory::Single,
            ValueType::Void => unimplemented!()
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            ValueType::Int => Int::default().into(),
            ValueType::Long => Long::default().into(),
            ValueType::Float => Float::default().into(),
            ValueType::Double => Double::default().into(),
            ValueType::Null => Reference::default().into(),
            ValueType::Reference(_) => Reference::default().into(),
            ValueType::AnyReference => Reference::default().into(),
            ValueType::Void => unimplemented!()
        }
    }
}


#[derive(Debug, Clone)]
pub enum Value {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}


impl Value {
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Int(_) => ValueType::Int,
            Value::Long(_) => ValueType::Long,
            Value::Float(_) => ValueType::Float,
            Value::Double(_) => ValueType::Double,
            Value::Reference(Reference::Null) => ValueType::Null,
            Value::Reference(Reference::Instance(instance)) => ValueType::Reference(instance.class().name().clone())
        }
    }
}


impl TryFrom<Value> for Int {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Int,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<Value> for Long {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Long(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Long,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<Value> for Float {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Float,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<Value> for Double {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Double(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Double,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<Value> for Reference {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Reference(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::AnyReference,
                found: value.value_type(),
            }),
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



impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Long(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Double(value) => write!(f, "{}", value),
            Value::Reference(value) => write!(f, "{}", value),
        }
    }
}
