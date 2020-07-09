use std::convert::TryFrom;
use std::fmt;

use itertools::join;

use crate::class::error::DescriptorError;
use crate::class::name::ClassName;
use crate::types::byte::Byte;
use crate::types::category::{Categorize, ValueCategory};
use crate::types::double::Double;
use crate::types::float::Float;
use crate::types::int::Int;
use crate::types::jvm_value::JvmValue;
use crate::types::long::Long;
use crate::types::reference::Reference;
use crate::types::short::Short;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeDescriptor {
    Simple(SimpleDescriptor),
    Array(ArrayDescriptor),
}


impl TypeDescriptor {
    pub fn default_value(&self) -> JvmValue {
        match self {
            TypeDescriptor::Simple(descriptor) => {
                match descriptor {
                    SimpleDescriptor::Byte => Byte::default().into(),
                    SimpleDescriptor::Double => Double::default().into(),
                    SimpleDescriptor::Float => Float::default().into(),
                    SimpleDescriptor::Int => Int::default().into(),
                    SimpleDescriptor::Long => Long::default().into(),
                    SimpleDescriptor::Reference(_) => Reference::default().into(),
                    SimpleDescriptor::Short => Short::default().into(),
                }
            }
            TypeDescriptor::Array(descriptor) => {
                Reference::default().into()
            }
        }
    }

    pub fn type_category(&self) -> ValueCategory {
        match self {
            TypeDescriptor::Simple(descriptor) => {
                match descriptor {
                    SimpleDescriptor::Byte => Byte::category(),
                    SimpleDescriptor::Double => Double::category(),
                    SimpleDescriptor::Float => Float::category(),
                    SimpleDescriptor::Int => Int::category(),
                    SimpleDescriptor::Long => Long::category(),
                    SimpleDescriptor::Reference(_) => Reference::category(),
                    SimpleDescriptor::Short => Short::category(),
                }
            }
            TypeDescriptor::Array(descriptor) => {
                Reference::category()
            }
        }
    }
}


impl fmt::Display for TypeDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeDescriptor::Simple(d) => write!(f, "{}", d),
            TypeDescriptor::Array(d) => write!(f, "{}", d),
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayDim(u8);


impl ArrayDim {
    pub fn new(dim: u8) -> Result<Self, DescriptorError> {
        if dim == 0 {
            return Err(DescriptorError::ZeroArrayDimension);
        }

        Ok(ArrayDim(dim))
    }
}


impl TryFrom<u8> for ArrayDim {
    type Error = DescriptorError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ArrayDim::new(value)
    }
}


impl AsRef<u8> for ArrayDim {
    fn as_ref(&self) -> &u8 {
        &self.0
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ArrayDescriptor {
    element_desc: SimpleDescriptor,
    dim: ArrayDim,
}


impl ArrayDescriptor {
    pub fn new(element_desc: SimpleDescriptor, dim: ArrayDim) -> Self {
        ArrayDescriptor {
            element_desc,
            dim,
        }
    }

    pub fn element_desc(&self) -> &SimpleDescriptor {
        &self.element_desc
    }

    pub fn dim(&self) -> &ArrayDim {
        &self.dim
    }
}


impl From<ArrayDescriptor> for TypeDescriptor {
    fn from(descriptor: ArrayDescriptor) -> Self {
        TypeDescriptor::Array(descriptor)
    }
}


impl fmt::Display for ArrayDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", "[]".repeat(*self.dim.as_ref() as usize), self.element_desc)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SimpleDescriptor {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference(ClassName),
}


impl fmt::Display for SimpleDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleDescriptor::Byte => write!(f, "byte"),
            SimpleDescriptor::Double => write!(f, "double"),
            SimpleDescriptor::Float => write!(f, "float"),
            SimpleDescriptor::Int => write!(f, "int"),
            SimpleDescriptor::Long => write!(f, "long"),
            SimpleDescriptor::Reference(c) => write!(f, "{}", c),
            SimpleDescriptor::Short => write!(f, "short"),
        }
    }
}


impl From<SimpleDescriptor> for TypeDescriptor {
    fn from(descriptor: SimpleDescriptor) -> Self {
        TypeDescriptor::Simple(descriptor)
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReturnDescriptor {
    Void,
    NonVoid(TypeDescriptor),
}


impl ReturnDescriptor {
    pub fn is_void(&self) -> bool {
        *self == ReturnDescriptor::Void
    }
}


impl From<TypeDescriptor> for ReturnDescriptor {
    fn from(descriptor: TypeDescriptor) -> Self {
        ReturnDescriptor::NonVoid(descriptor)
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
