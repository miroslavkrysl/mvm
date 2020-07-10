//! Field and method signatures.

use std::fmt;
use std::fmt::Display;

use crate::class::descriptor::{ParamsDesc, ReturnDesc, TypeDesc};
use crate::class::error::SignatureError;
use crate::class::name::{FieldName, MethodName};


/// A method signature consisting of a method return type descriptor,
/// a method name and a method parameters types descriptor.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MethodSig {
    return_desc: ReturnDesc,
    name: MethodName,
    params_desc: ParamsDesc,
}


impl MethodSig {
    /// Creates a new `MethodSig` with the given return type descriptor,
    /// method name and method parameters types descriptor.
    ///
    /// # Errors
    ///
    /// Returns `SignatureError::InitIsNonVoid` if the method name is of instance initialization method
    /// and the return type is not void
    /// or `SignatureError::ClinitIsNonVoid` if the method name is of class initialization method
    /// and the return type is not void.
    pub fn new(return_desc: ReturnDesc, name: MethodName, params_desc: ParamsDesc) -> Result<Self, SignatureError> {
        if name.is_init() && !return_desc.is_void() {
            return Err(SignatureError::InitIsNonVoid);
        }

        if name.is_clinit() && !return_desc.is_void() {
            return Err(SignatureError::ClinitIsNonVoid);
        }

        Ok(MethodSig {
            name,
            return_desc,
            params_desc,
        })
    }

    /// Returns the the method return type descriptor.
    pub fn return_desc(&self) -> &ReturnDesc {
        &self.return_desc
    }

    /// Returns the method name.
    pub fn name(&self) -> &MethodName {
        &self.name
    }

    /// Returns the parameters types descriptor of the method.
    pub fn params_desc(&self) -> &ParamsDesc {
        &self.params_desc
    }
}


impl MethodSig {
    /// Returns true if this signature can be an instance initialization method, false otherwise.
    pub fn is_init(&self) -> bool {
        self.name.is_init() && self.return_desc.is_void()
    }

    /// Returns true if this signature can be an instance initialization method, false otherwise.
    pub fn is_clinit(&self) -> bool {
        self.name.is_init() && self.return_desc.is_void()
    }
}


impl Display for MethodSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}({})", self.return_desc, self.name, self.params_desc)
    }
}


/// A field signature consisting of a type descriptor and a field name.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldSig {
    type_desc: TypeDesc,
    name: FieldName,
}


impl FieldSig {
    /// Creates a new `FieldSig` with the given return type descriptor,
    /// method name and parameters types descriptor.
    pub fn new(type_desc: TypeDesc, name: FieldName) -> Self {
        FieldSig {
            type_desc,
            name,
        }
    }

    /// Returns the the field type descriptor.
    pub fn type_desc(&self) -> &TypeDesc {
        &self.type_desc
    }

    /// Returns the the field name.
    pub fn name(&self) -> &FieldName {
        &self.name
    }
}


impl Display for FieldSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.type_desc, self.name)
    }
}