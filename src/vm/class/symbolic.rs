//! Symbolic references to fields and methods.

use std::fmt;

use crate::vm::class::name::ClassName;
use crate::vm::class::signature::{FieldSig, MethodSig};


/// A symbolic reference to a class field with a specific signature.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldRef {
    class_name: ClassName,
    signature: FieldSig,
}


impl FieldRef {
    /// Creates a new `FieldRef` with the given class name and signature.
    pub fn new(class_name: ClassName, signature: FieldSig) -> Self {
        FieldRef {
            class_name,
            signature,
        }
    }

    /// Returns the name of the class of the referenced field
    pub fn class_name(&self) -> &ClassName {
        &self.class_name
    }

    /// Returns the signature of the referenced field.
    pub fn signature(&self) -> &FieldSig {
        &self.signature
    }
}


impl fmt::Display for FieldRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.signature.type_desc(), self.class_name, self.signature.name())
    }
}


/// A symbolic reference to a class method with a specific signature.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodRef {
    class_name: ClassName,
    signature: MethodSig,
}


impl MethodRef {
    /// Creates a new `MethodRef` with the given class name and signature.
    pub fn new(class_name: ClassName, signature: MethodSig) -> Self {
        MethodRef {
            class_name,
            signature,
        }
    }

    /// Returns the name of the class of the referenced method.
    pub fn class_name(&self) -> &ClassName {
        &self.class_name
    }

    /// Returns the signature of the referenced method.
    pub fn signature(&self) -> &MethodSig {
        &self.signature
    }
}


impl fmt::Display for MethodRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}({})",
               self.signature.return_desc(),
               self.class_name, self.signature.name(),
               self.signature.params_desc())
    }
}
