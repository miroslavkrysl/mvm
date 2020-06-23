use crate::class::{MethodName, MethodFlags, MethodDescriptor, Code};

#[derive(Debug, Clone)]
pub struct Method {
    name: MethodName,
    flags: MethodFlags,
    descriptor: MethodDescriptor,
    code: Option<Code>,
}

impl Method {
    pub fn new(name: MethodName, flags: MethodFlags, descriptor: MethodDescriptor, code: Option<Code>) -> Self {
        Method { name, flags, descriptor, code }
    }

    pub fn flags(&self) -> &MethodFlags {
        &self.flags
    }

    pub fn name(&self) -> &MethodName {
        &self.name
    }

    pub fn descriptor(&self) -> &MethodDescriptor {
        &self.descriptor
    }

    pub fn code(&self) -> Option<&Code> {
        self.code.as_ref()
    }
}
