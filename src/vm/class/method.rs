use crate::vm::class::code::Code;
use crate::vm::class::error::MethodError;
use crate::vm::class::signature::MethodSig;
use crate::vm::types::value::ValueType;


/// A field of a class.
/// Can be static (class field) or non-static (instance field).
#[derive(Debug, Clone)]
pub struct Method {
    signature: MethodSig,
    is_static: bool,
    code: Code,
}


impl Method {
    /// Creates a new method with the given signature and code.
    ///
    /// # Errors
    ///
    /// Returns `MethodError::TooFewLocalsEntries` if the method locals size is smaller
    /// than required for method parameters,
    /// `MethodError::InitIsStatic` if the method signature is of instance initialization method
    /// or `MethodError::ClinitIsNonStatic` if the method signature is of class initialization method.
    pub fn new(signature: MethodSig, is_static: bool, code: Code) -> Result<Self, MethodError> {
        if signature.is_init() && is_static {
            return Err(MethodError::InitIsStatic);
        }

        if signature.is_clinit() && !is_static {
            return Err(MethodError::ClinitIsNonStatic);
        }

        let mut params_size = signature.params_desc().size();

        if !is_static {
            params_size += ValueType::AnyReference.category().size();
        }

        if code.locals_size() <  params_size {
            return Err(MethodError::TooFewLocalsEntries {
                locals_size: code.locals_size(),
                params_size,
            });
        }

        Ok(Method {
            signature,
            is_static,
            code,
        })
    }

    /// Returns the method signature.
    pub fn signature(&self) -> &MethodSig {
        &self.signature
    }

    /// Returns the method code.
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Returns true if this method is static, false otherwise.
    pub fn is_static(&self) -> bool {
        self.is_static
    }
}


impl Method {
    /// Returns true if this method is an instance initialization method, false otherwise.
    pub fn is_init(&self) -> bool {
        self.signature.is_init() && self.is_static
    }

    /// Returns true if this method is an instance initialization method, false otherwise.
    pub fn is_clinit(&self) -> bool {
        self.signature.is_init() && self.is_static
    }
}