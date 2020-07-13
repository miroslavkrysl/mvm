//! Value types, method return types and method parameters descriptors.

use std::fmt;
use std::iter::FromIterator;
use crate::vm::class::name::ClassName;
use crate::vm::types::value::{ValueType, Value};
use crate::vm::types::reference::Reference;
use crate::vm::class::instance::Instance;
use itertools::join;


/// A MVM type descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeDesc {
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Reference(ClassName),
}

impl TypeDesc {
    /// Returns the default value of the `MvmValue` type described by this descriptor.
    pub fn value_type(&self) -> ValueType {
        match self {
            TypeDesc::Byte => ValueType::Byte,
            TypeDesc::Short => ValueType::Short,
            TypeDesc::Int => ValueType::Int,
            TypeDesc::Long => ValueType::Long,
            TypeDesc::Float => ValueType::Float,
            TypeDesc::Double => ValueType::Double,
            TypeDesc::Reference(_) => ValueType::Reference,
        }
    }

    /// Returns the true if the given value can be assigned into field of this descriptor.
    pub fn is_assignable_with(&self, value: &Value) -> bool {
        match value {
            Value::Byte(_) => self.is_byte(),
            Value::Short(_) => self.is_short(),
            Value::Int(_) => self.is_int(),
            Value::Long(_) => self.is_long(),
            Value::Float(_) => self.is_float(),
            Value::Double(_) => self.is_double(),
            Value::Reference(reference) => {
                match reference {
                    Reference::Null => true,
                    Reference::Instance(instance) => self.is_reference_to_instance(instance),
                }
            }
        }
    }

    /// Returns the true if this descriptor describes a byte value.
    pub fn is_byte(&self) -> bool {
        match self {
            TypeDesc::Byte => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a short value.
    pub fn is_short(&self) -> bool {
        match self {
            TypeDesc::Short => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes an int value.
    pub fn is_int(&self) -> bool {
        match self {
            TypeDesc::Int => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a long value.
    pub fn is_long(&self) -> bool {
        match self {
            TypeDesc::Long => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a float value.
    pub fn is_float(&self) -> bool {
        match self {
            TypeDesc::Float => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a double value.
    pub fn is_double(&self) -> bool {
        match self {
            TypeDesc::Double => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a reference value.
    pub fn is_reference(&self) -> bool {
        match self {
            TypeDesc::Reference(_) => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor references the given instance.
    pub fn is_reference_to_instance(&self, instance: &Instance) -> bool {
        match self {
            TypeDesc::Reference(class_name) => class_name == instance.class().name(),
            _ => false
        }
    }
}



impl fmt::Display for TypeDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeDesc::Byte => write!(f, "byte"),
            TypeDesc::Short => write!(f, "short"),
            TypeDesc::Int => write!(f, "int"),
            TypeDesc::Long => write!(f, "long"),
            TypeDesc::Float => write!(f, "float"),
            TypeDesc::Double => write!(f, "double"),
            TypeDesc::Reference(c) => write!(f, "{}", c),
        }
    }
}


/// A method return type descriptor.
/// Can be `Void` or `NonVoid`, than it has an inner type descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReturnDesc {
    Void,
    NonVoid(TypeDesc),
}


impl ReturnDesc {
    /// Returns true if the return descriptor is void.
    pub fn is_void(&self) -> bool {
        *self == ReturnDesc::Void
    }
}


impl From<TypeDesc> for ReturnDesc {
    fn from(descriptor: TypeDesc) -> Self {
        ReturnDesc::NonVoid(descriptor)
    }
}


impl fmt::Display for ReturnDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReturnDesc::Void => write!(f, "void"),
            ReturnDesc::NonVoid(d) => write!(f, "{}", d),
        }
    }
}


/// A method parameters types descriptor.
/// It is a list of parameter types descriptors.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ParamsDesc {
    params_desc: Vec<TypeDesc>,
}


impl ParamsDesc {
    /// Creates a descriptor with no parameters.
    pub fn empty() -> Self {
        ParamsDesc { params_desc: Vec::new() }
    }

    /// Returns the number of the parameters in this descriptor.
    pub fn len(&self) -> usize {
        self.params_desc.len()
    }

    /// Returns the total size of the parameters according to the value category
    /// of their type descriptors.
    pub fn size(&self) -> usize {
        self.params_desc.iter().map(|d| d.value_type().category().size()).sum()
    }
}


impl FromIterator<TypeDesc> for ParamsDesc {
    fn from_iter<T: IntoIterator<Item=TypeDesc>>(iter: T) -> Self {
        ParamsDesc {
            params_desc: iter.into_iter().collect()
        }
    }
}


impl fmt::Display for ParamsDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", join(&self.params_desc, ", "))
    }
}