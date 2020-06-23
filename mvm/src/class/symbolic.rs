use std::fmt;
use crate::class::{ClassName, FieldName, FieldDescriptor, MethodName, MethodDescriptor};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ClassSymRef {
    name: ClassName,
}

impl ClassSymRef {
    pub fn new(name: ClassName) -> Self {
        ClassSymRef { name }
    }
}

impl fmt::Display for ClassSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldSymRef {
    class: ClassSymRef,
    id: FieldId
}

impl FieldSymRef {
    pub fn new(class: ClassSymRef, id: FieldId) -> Self {
        FieldSymRef { class, id }
    }
    
    pub fn class(&self) -> &ClassSymRef {
        &self.class
    }
    
    pub fn id(&self) -> &FieldId {
        &self.id
    }
}

impl fmt::Display for FieldSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.class, self.id.name(), self.id.descriptor())
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldId {
    name: FieldName,
    descriptor: FieldDescriptor
}

impl FieldId {
    pub fn new(name: FieldName, descriptor: FieldDescriptor) -> Self {
        FieldId { name, descriptor }
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn descriptor(&self) -> &FieldDescriptor {
        &self.descriptor
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodSymRef {
    class: ClassSymRef,
    id: MethodId
}

impl MethodSymRef {
    pub fn new(class: ClassSymRef, id: MethodId) -> Self {
        MethodSymRef { class, id }
    }

    pub fn class(&self) -> &ClassSymRef {
        &self.class
    }

    pub fn id(&self) -> &MethodId {
        &self.id
    }
}

impl fmt::Display for MethodSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.class, self.id.name(), self.id.descriptor())
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MethodId {
    name: MethodName,
    descriptor: MethodDescriptor
}

impl MethodId {
    pub fn new(name: MethodName, descriptor: MethodDescriptor) -> Self {
        MethodId { name, descriptor }
    }

    pub fn name(&self) -> &MethodName {
        &self.name
    }

    pub fn descriptor(&self) -> &MethodDescriptor {
        &self.descriptor
    }
}