//! Value types, method return types and method parameters descriptors.

use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;

use itertools::join;

use crate::class::error::DescriptorError;
use crate::class::name::ClassName;
use crate::class::object::{Array, Object};
use crate::types::byte::Byte;
use crate::types::category::{Categorize, ValueCategory};
use crate::types::double::Double;
use crate::types::float::Float;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::reference::Reference;
use crate::types::short::Short;
use crate::types::value::Value;


/// A MVM type descriptor.
/// Can be `Simple` - primitive or class type descriptor,
/// or `Array` - array type descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TypeDesc {
    Simple(SimpleDescriptor),
    Array(ArrayDesc),
}


impl TypeDesc {
    /// Returns the default value of the `MvmValue` type described by this descriptor.
    pub fn default_value(&self) -> Value {
        match self {
            TypeDesc::Simple(descriptor) => {
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
            TypeDesc::Array(descriptor) => {
                Reference::default().into()
            }
        }
    }

    /// Returns the value category of the `JvmValue` type described by this descriptor.
    pub fn value_category(&self) -> ValueCategory {
        match self {
            TypeDesc::Simple(descriptor) => {
                match descriptor {
                    SimpleDescriptor::Byte => Byte::category(),
                    SimpleDescriptor::Short => Short::category(),
                    SimpleDescriptor::Int => Int::category(),
                    SimpleDescriptor::Long => Long::category(),
                    SimpleDescriptor::Float => Float::category(),
                    SimpleDescriptor::Double => Double::category(),
                    SimpleDescriptor::Reference(_) => Reference::category(),
                }
            }
            TypeDesc::Array(descriptor) => {
                Reference::category()
            }
        }
    }

    /// Returns the true if this descriptor describes the given value.
    pub fn describes(&self, value: &Value) -> bool {
        // TODO implement describing
        unimplemented!()
        // match value {
        //     Value::Byte(_) => self.is_byte(),
        //     Value::Short(_) => self.is_byte(),
        //     Value::Int(_) => self.is_byte(),
        //     Value::Long(_) => self.is_byte(),
        //     Value::Float(_) => self.is_byte(),
        //     Value::Double(_) => self.is_byte(),
        //     Value::Reference(reference) => {
        //         match reference {
        //             Reference::Null => true,
        //             Reference::Object(object) => match object.as_ref() {
        //                     Object::Instance(instance) => match self {
        //                         TypeDesc::Simple(SimpleDescriptor::Reference(class_name)) => {
        //                             instance.class().name() == class_name
        //                         }
        //                         _ => false
        //                     },
        //                     Object::Array(array) => match self {
        //                         TypeDesc::Array(array_desc) => {
        //                             match array {
        //                                 Array::Byte(_) => array_desc.is_byte(),
        //                                 Array::Short(_) => {},
        //                                 Array::Int(_) => {},
        //                                 Array::Long(_) => {},
        //                                 Array::Float(_) => {},
        //                                 Array::Double(_) => {},
        //                                 Array::Reference(_) => {},
        //                             }
        //                         }
        //                         _ => false
        //                     },
        //             }
        //         }
        //     }
        // }
    }

    /// Returns the true if this descriptor describes a byte value.
    pub fn is_byte(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_byte(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a short value.
    pub fn is_short(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_short(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes an int value.
    pub fn is_int(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_int(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a long value.
    pub fn is_long(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_long(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a float value.
    pub fn is_float(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_float(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a double value.
    pub fn is_double(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_double(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a reference value.
    pub fn is_reference(&self) -> bool {
        match self {
            TypeDesc::Simple(s) => s.is_reference(),
            _ => false
        }
    }

    /// Returns the true if this descriptor describes an array value.
    pub fn is_array(&self) -> bool {
        match self {
            TypeDesc::Array(_) => true,
            _ => false
        }
    }
}


impl From<SimpleDescriptor> for TypeDesc {
    fn from(descriptor: SimpleDescriptor) -> Self {
        TypeDesc::Simple(descriptor)
    }
}


impl fmt::Display for TypeDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeDesc::Simple(d) => write!(f, "{}", d),
            TypeDesc::Array(d) => write!(f, "{}", d),
        }
    }
}


/// The number of array dimensions.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct ArrayDim(usize);


impl ArrayDim {
    /// Creates a new array dimensions number.
    ///
    /// # Errors
    ///
    /// Returns `DescriptorError::ZeroArrayDimensions` if the `dim` is 0
    /// or `DescriptorError::TooManyDimensions` if the `dim` is greater than allowed.
    pub fn new(dim: usize) -> Result<Self, DescriptorError> {
        if dim == 0 {
            return Err(DescriptorError::ZeroArrayDimensions);
        }

        if dim > Array::MAX_DIM {
            return Err(DescriptorError::TooManyDimensions {
                max: Array::MAX_DIM,
                dim,
            });
        }

        Ok(ArrayDim(dim))
    }
}


impl TryFrom<usize> for ArrayDim {
    type Error = DescriptorError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        ArrayDim::new(value)
    }
}


impl AsRef<usize> for ArrayDim {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}


impl fmt::Display for ArrayDim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// An array type descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ArrayDesc {
    type_desc: SimpleDescriptor,
    dim: ArrayDim,
}


impl ArrayDesc {
    /// Creates a new `ArrayDesc` with the given element type descriptor
    /// and dimensions.
    pub fn new(type_desc: SimpleDescriptor, dim: ArrayDim) -> Self {
        ArrayDesc {
            type_desc,
            dim,
        }
    }

    /// Returns the element type descriptor.
    pub fn type_desc(&self) -> &SimpleDescriptor {
        &self.type_desc
    }

    /// Returns the number of dimensions.
    pub fn dim(&self) -> &ArrayDim {
        &self.dim
    }
}


impl From<ArrayDesc> for TypeDesc {
    fn from(descriptor: ArrayDesc) -> Self {
        TypeDesc::Array(descriptor)
    }
}


impl fmt::Display for ArrayDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.dim, self.type_desc)
    }
}


/// A primitive type or class reference type descriptor.
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

impl SimpleDescriptor {
    /// Returns the true if this descriptor describes a byte value.
    pub fn is_byte(&self) -> bool {
        match self {
            SimpleDescriptor::Byte => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a short value.
    pub fn is_short(&self) -> bool {
        match self {
            SimpleDescriptor::Short => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes an int value.
    pub fn is_int(&self) -> bool {
        match self {
            SimpleDescriptor::Int => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a long value.
    pub fn is_long(&self) -> bool {
        match self {
            SimpleDescriptor::Long => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a float value.
    pub fn is_float(&self) -> bool {
        match self {
            SimpleDescriptor::Float => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a double value.
    pub fn is_double(&self) -> bool {
        match self {
            SimpleDescriptor::Double => true,
            _ => false
        }
    }

    /// Returns the true if this descriptor describes a reference value.
    pub fn is_reference(&self) -> bool {
        match self {
            SimpleDescriptor::Reference(_) => true,
            _ => false
        }
    }
}



impl fmt::Display for SimpleDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleDescriptor::Byte => write!(f, "byte"),
            SimpleDescriptor::Short => write!(f, "short"),
            SimpleDescriptor::Int => write!(f, "int"),
            SimpleDescriptor::Long => write!(f, "long"),
            SimpleDescriptor::Float => write!(f, "float"),
            SimpleDescriptor::Double => write!(f, "double"),
            SimpleDescriptor::Reference(c) => write!(f, "{}", c),
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
        self.params_desc.iter().map(|d| d.value_category().size()).sum()
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