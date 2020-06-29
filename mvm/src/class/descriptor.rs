use crate::class::symbolic::ClassSymRef;
use std::fmt;
use itertools::join;
use crate::types::{JvmValue, Byte, Char, Double, Float, Long, Reference, Boolean, Short, Int};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ValueDescriptor {
    Simple(SimpleValueDescriptor),
    Array(ArrayValueDescriptor),
}

impl ValueDescriptor {
    pub fn default_value(&self) -> JvmValue {
        match self {
            ValueDescriptor::Simple(descriptor) => {
                match descriptor {
                    SimpleValueDescriptor::Byte => Byte::default().into(),
                    SimpleValueDescriptor::Char => Char::default().into(),
                    SimpleValueDescriptor::Double => Double::default().into(),
                    SimpleValueDescriptor::Float => Float::default().into(),
                    SimpleValueDescriptor::Int => Int::default().into(),
                    SimpleValueDescriptor::Long => Long::default().into(),
                    SimpleValueDescriptor::Reference(_) => Reference::default().into(),
                    SimpleValueDescriptor::Short => Short::default().into(),
                    SimpleValueDescriptor::Boolean => Boolean::default().into(),
                }
            },
            ValueDescriptor::Array(descriptor) => {
                Reference::default().into()
            },
        }
    }
}

impl fmt::Display for ValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueDescriptor::Simple(d) => write!(f, "{}", d),
            ValueDescriptor::Array(d) => write!(f, "{}", d),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ArrayValueDescriptor {
    dim: u8,
    values_desc: SimpleValueDescriptor,
}

impl ArrayValueDescriptor {
    pub fn dim(&self) -> &u8 {
        &self.dim
    }

    pub fn values_desc(&self) -> &SimpleValueDescriptor {
        &self.values_desc
    }
}

impl fmt::Display for ArrayValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", "[".repeat(self.dim as usize), self.values_desc)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimpleValueDescriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Reference(ClassSymRef),
    Short,
    Boolean,
}

impl fmt::Display for SimpleValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleValueDescriptor::Byte => write!(f, "boolean"),
            SimpleValueDescriptor::Char => write!(f, "char"),
            SimpleValueDescriptor::Double => write!(f, "Double"),
            SimpleValueDescriptor::Float => write!(f, "float"),
            SimpleValueDescriptor::Int => write!(f, "int"),
            SimpleValueDescriptor::Long => write!(f, "long"),
            SimpleValueDescriptor::Reference(c) => write!(f, "&{}", c),
            SimpleValueDescriptor::Short => write!(f, "short"),
            SimpleValueDescriptor::Boolean => write!(f, "boolean"),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MethodDescriptor {
    params_descs: Vec<ValueDescriptor>,
    return_desc: ReturnDescriptor,
}

impl MethodDescriptor {
    pub fn params_descs(&self) -> impl ExactSizeIterator<Item=&ValueDescriptor> {
        self.params_descs.iter()
    }

    pub fn return_desc(&self) -> &ReturnDescriptor {
        &self.return_desc
    }
}

impl fmt::Display for MethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", join(&self.params_descs, ""), self.return_desc)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReturnDescriptor {
    Void,
    NonVoid(ValueDescriptor),
}

impl ReturnDescriptor {
    pub fn is_void(&self) -> bool {
        *self == ReturnDescriptor::Void
    }
}

impl fmt::Display for ReturnDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnDescriptor::Void => write!(f, "V"),
            ReturnDescriptor::NonVoid(d) => write!(f, "{}", d),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldDescriptor {
    value_desc: ValueDescriptor,
}

impl FieldDescriptor {
    pub fn value_desc(&self) -> &ValueDescriptor {
        &self.value_desc
    }
}

impl fmt::Display for FieldDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value_desc)
    }
}


