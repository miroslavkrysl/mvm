use std::sync::Arc;

use crate::class::{Class, Code, MethodDescriptor, MethodError, MethodFlags, MethodName};

#[derive(Debug, Clone)]
pub struct Method {
    class: Arc<Class>,
    name: MethodName,
    flags: MethodFlags,
    descriptor: MethodDescriptor,
    code: Code,
}

impl Method {
    pub fn new(class: Arc<Class>, name: MethodName, flags: MethodFlags, descriptor: MethodDescriptor, code: Code) -> Result<Self, MethodError> {
        if name.is_init() {
            if !class.is_class() || !descriptor.return_desc().is_void() {
                return Err(MethodError::InvalidInitProperties);
            }
        } else if name.is_class_init() {
            if !descriptor.return_desc().is_void() || descriptor.params_descs().len() == 0 || !flags.has(MethodFlags::STATIC) {
                return Err(MethodError::InvalidInitProperties);
            }
        }

        // TODO: check interface methods constraints
        // TODO: check abstract methods constraints

        Ok(Method {
            class,
            name,
            flags,
            descriptor,
            code,
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

    pub fn descriptor(&self) -> &MethodDescriptor {
        &self.descriptor
    }

    pub fn code(&self) -> &Code {
        &self.code
    }
}

/// Method access and properties related logic.
impl Method {
    pub fn is_static(&self) -> bool {
        self.flags.has(MethodFlags::STATIC)
    }

    pub fn is_private(&self) -> bool {
        self.flags.has(MethodFlags::PRIVATE)
    }

    pub fn is_final(&self) -> bool {
        self.flags.has(MethodFlags::FINAL)
    }

    pub fn is_init(&self) -> bool {
        self.name.is_init()
    }

    pub fn is_class_init(&self) -> bool {
        self.name.is_class_init()
    }

    pub fn is_accessible_for(&self, class: &Class) -> bool {
        // TODO: implement inter-class accessibility
        true
    }

    pub fn overrides(&self, method: &Method) -> bool {
        self.name == method.name &&
            self.descriptor == method.descriptor &&
            self.is_accessible_for(method.class.as_ref())
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.descriptor == other.descriptor && self.class == other.class
    }
}

impl Eq for Method {}