use crate::class::name::MethodName;
use crate::class::flags::MethodFlags;
use crate::class::code::Code;
use crate::class::error::MethodError;
use crate::class::class::Class;
use crate::class::descriptor::{ReturnDescriptor, TypeDescriptor};


#[derive(Debug, Clone)]
pub struct Method {
    name: MethodName,
    return_desc: ReturnDescriptor,
    params_desc: Vec<TypeDescriptor>,
    flags: MethodFlags,
    code: Code,
}

impl Method {
    pub fn new(
        name: MethodName,
        return_desc: ReturnDescriptor,
        params_desc: Vec<TypeDescriptor>,
        flags: MethodFlags,
        code: Code,
    ) -> Result<Self, MethodError> {
        if code.max_locals() < params_desc.len() {
            return Err(MethodError::TooFewLocalsEntries)
        }

        Ok(Method {
            name,
            return_desc,
            params_desc,
            flags,
            code
        })
    }
}

/// Getters
impl Method {
    pub fn flags(&self) -> &MethodFlags {
        &self.flags
    }

    pub fn name(&self) -> &MethodName {
        &self.name
    }

    pub fn return_desc(&self) -> &ReturnDescriptor {
        &self.return_desc
    }

    pub fn params_desc(&self) -> &Vec<TypeDescriptor> {
        &self.params_desc
    }

    pub fn code(&self) -> &Code {
        &self.code
    }
}

/// Method access and properties related logic.
impl Method {
    pub fn is_static(&self) -> bool {
        self.flags.is_static()
    }

    pub fn is_init(&self) -> bool {
        self.name.is_init()
    }

    pub fn is_class_init(&self) -> bool {
        self.name.is_class_init()
    }
}