use std::fmt;
use crate::class::descriptor::{ValueDescriptor, MethodDescriptor};
use crate::class::name::{FieldName, ClassName, MethodName};
use crate::class::FieldDescriptor;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    name: FieldName,
    descriptor: FieldDescriptor
}

impl FieldSymRef {
    pub fn new(class: ClassSymRef, name: FieldName, descriptor: FieldDescriptor) -> Self {
        FieldSymRef { class, name, descriptor }
    }
}

impl fmt::Display for FieldSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.class, self.name, self.descriptor)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodSymRef {
    class: ClassSymRef,
    name: MethodName,
    descriptor: MethodDescriptor
}

impl MethodSymRef {
    pub fn new(class: ClassSymRef, name: MethodName, descriptor: MethodDescriptor) -> Self {
        MethodSymRef { class, name, descriptor }
    }
}

impl fmt::Display for MethodSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{} {}", self.class, self.name, self.descriptor)
    }
}
