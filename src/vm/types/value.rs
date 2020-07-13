use std::convert::TryFrom;
use std::fmt;

use crate::vm::types::byte::Byte;
use crate::vm::types::double::Double;
use crate::vm::types::error::ValueError;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;
use crate::vm::types::short::Short;
use std::fmt::Debug;


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
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Byte => write!(f, "byte"),
            ValueType::Short => write!(f, "short"),
            ValueType::Int => write!(f, "int"),
            ValueType::Long => write!(f, "long"),
            ValueType::Float => write!(f, "float"),
            ValueType::Double => write!(f, "double"),
            ValueType::Reference => write!(f, "reference"),
        }
    }
}

impl ValueType {
    pub fn category(&self) -> ValueCategory {
        match self {
            ValueType::Byte => ValueCategory::Single,
            ValueType::Short => ValueCategory::Single,
            ValueType::Int => ValueCategory::Single,
            ValueType::Long => ValueCategory::Double,
            ValueType::Float => ValueCategory::Single,
            ValueType::Double => ValueCategory::Double,
            ValueType::Reference => ValueCategory::Single,
        }
    }

    pub fn default_value(&self) -> Value {
        match self {
            ValueType::Byte => Byte::default().into(),
            ValueType::Short => Short::default().into(),
            ValueType::Int => Int::default().into(),
            ValueType::Long => Long::default().into(),
            ValueType::Float => Float::default().into(),
            ValueType::Double => Double::default().into(),
            ValueType::Reference => Reference::default().into(),
        }
    }
}


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
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Byte(_) => ValueType::Byte,
            Value::Short(_) => ValueType::Short,
            Value::Int(_) => ValueType::Int,
            Value::Long(_) => ValueType::Long,
            Value::Float(_) => ValueType::Float,
            Value::Double(_) => ValueType::Double,
            Value::Reference(_) => ValueType::Reference
        }
    }
}


impl TryFrom<Value> for Byte {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Byte(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Byte,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<Value> for Short {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Short(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Short,
                found: value.value_type(),
            }),
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
                expected: ValueType::Reference,
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
            Value::Byte(value) => write!(f, "{}", value),
            Value::Short(value) => write!(f, "{}", value),
            Value::Int(value) => write!(f, "{}", value),
            Value::Long(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Double(value) => write!(f, "{}", value),
            Value::Reference(value) => write!(f, "{}", value),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum CompValue {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}


impl CompValue {
    pub fn value_type(&self) -> ValueType {
        match self {
            CompValue::Int(_) => ValueType::Int,
            CompValue::Long(_) => ValueType::Long,
            CompValue::Float(_) => ValueType::Float,
            CompValue::Double(_) => ValueType::Double,
            CompValue::Reference(_) => ValueType::Reference,
        }
    }
}


impl TryFrom<CompValue> for Int {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Int(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Int,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<CompValue> for Long {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Long(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Long,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<CompValue> for Float {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Float(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Float,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<CompValue> for Double {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Double(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Double,
                found: value.value_type(),
            }),
        }
    }
}


impl TryFrom<CompValue> for Reference {
    type Error = ValueError;

    fn try_from(value: CompValue) -> Result<Self, Self::Error> {
        match value {
            CompValue::Reference(v) => Ok(v),
            _ => Err(ValueError::TypeMismatch {
                expected: ValueType::Reference,
                found: value.value_type(),
            }),
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


impl From<CompValue> for Value {
    fn from(value: CompValue) -> Self {
        match value {
            CompValue::Int(v) => Value::Int(v),
            CompValue::Long(v) => Value::Long(v),
            CompValue::Float(v) => Value::Float(v),
            CompValue::Double(v) => Value::Double(v),
            CompValue::Reference(v) => Value::Reference(v),
        }
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





// fn try_into_reference(self, class_name: &ClassName) -> Result<Reference, ValueError> {
//     match self {
//         CompValue::Reference(r @ Reference::Null) => {
//             Ok(r)
//         }
//         CompValue::Reference(r @ Reference::Instance(ref i))
//         if i.class().name() == class_name => {
//             Ok(r)
//         }
//         _ => {
//             Err(ValueError::TypeMismatch {
//                 expected: ValueType::Reference(class_name.clone()),
//                 found: self.value_type(),
//             })
//         }
//     }
// }




// pub fn is_byte(&self) -> bool {
//     if let Value::Byte(_) = self { true } else { false }
// }
//
// pub fn is_short(&self) -> bool {
//     if let Value::Short(_) = self { true } else { false }
// }
//
// pub fn is_int(&self) -> bool {
//     if let Value::Int(_) = self { true } else { false }
// }
//
// pub fn is_long(&self) -> bool {
//     if let Value::Long(_) = self { true } else { false }
// }
//
// pub fn is_float(&self) -> bool {
//     if let Value::Float(_) = self { true } else { false }
// }
//
// pub fn is_double(&self) -> bool {
//     if let Value::Double(_) = self { true } else { false }
// }
//
// pub fn is_reference(&self) -> bool {
//     if let Value::Reference(_) = self { true } else { false }
// }