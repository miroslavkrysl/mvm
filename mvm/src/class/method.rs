use crate::class::name::MethodName;
use crate::class::flags::MethodFlags;
use crate::class::code::Code;
use crate::class::error::MethodError;
use crate::class::descriptor::{ReturnDescriptor, TypeDescriptor, ParamsDescriptor};


#[derive(Debug, Clone)]
pub struct Method {
    name: MethodName,
    return_desc: ReturnDescriptor,
    params_desc: ParamsDescriptor,
    is_static: bool,
    code: Code,
}

impl Method {
    pub fn new(
        name: MethodName,
        return_desc: ReturnDescriptor,
        params_desc: ParamsDescriptor,
        is_static: bool,
        code: Code,
    ) -> Result<Self, MethodError> {
        if code.locals_len() < params_desc.len() {
            return Err(MethodError::TooFewLocalsEntries)
        }

        Ok(Method {
            name,
            return_desc,
            params_desc,
            is_static,
            code
        })
    }
}

/// Getters
impl Method {
    pub fn name(&self) -> &MethodName {
        &self.name
    }

    pub fn return_desc(&self) -> &ReturnDescriptor {
        &self.return_desc
    }

    pub fn params_desc(&self) -> &ParamsDescriptor {
        &self.params_desc
    }

    pub fn code(&self) -> &Code {
        &self.code
    }
}

/// Method access and properties related logic.
impl Method {
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn is_init(&self) -> bool {
        self.name.is_init()
    }

    pub fn is_class_init(&self) -> bool {
        self.name.is_class_init()
    }
}