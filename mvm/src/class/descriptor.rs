use crate::class::symbolic::ClassSymRef;
use std::fmt;
use itertools::join;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueDescriptor {
    Simple(SimpleValueDescriptor),
    Array(ArrayValueDescriptor),
}

impl fmt::Display for ValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueDescriptor::Simple(d) => write!(f, "{}", d),
            ValueDescriptor::Array(d) => write!(f, "{}", d),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayValueDescriptor {
    dim: u8,
    type_desc: SimpleValueDescriptor,
}

impl ArrayValueDescriptor {
    pub fn dim(&self) -> &u8 {
        &self.dim
    }

    pub fn type_desc(&self) -> &SimpleValueDescriptor {
        &self.type_desc
    }
}

impl fmt::Display for ArrayValueDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", "[".repeat(self.dim as usize), self.type_desc)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
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
            SimpleValueDescriptor::Byte => write!(f, "B"),
            SimpleValueDescriptor::Char => write!(f, "C"),
            SimpleValueDescriptor::Double => write!(f, "D"),
            SimpleValueDescriptor::Float => write!(f, "F"),
            SimpleValueDescriptor::Int => write!(f, "I"),
            SimpleValueDescriptor::Long => write!(f, "J"),
            SimpleValueDescriptor::Reference(c) => write!(f, "L{}", c),
            SimpleValueDescriptor::Short => write!(f, "S"),
            SimpleValueDescriptor::Boolean => write!(f, "Z"),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodDescriptor {
    params_desc: Vec<ValueDescriptor>,
    return_desc: ReturnDescriptor,
}

impl MethodDescriptor {
    pub fn params_desc(&self) -> &Vec<ValueDescriptor> {
        &self.params_desc
    }

    pub fn return_desc(&self) -> &ReturnDescriptor {
        &self.return_desc
    }
}

impl fmt::Display for MethodDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}) {}", join(&self.params_desc, ""), self.return_desc)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReturnDescriptor {
    Void,
    NonVoid(ValueDescriptor),
}

impl fmt::Display for ReturnDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnDescriptor::Void => write!(f, "V"),
            ReturnDescriptor::NonVoid(d) => write!(f, "{}", d),
        }
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
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


