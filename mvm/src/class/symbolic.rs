use std::fmt;
use crate::class::name::{ClassName, FieldName, MethodName};
use crate::class::descriptor::{TypeDescriptor, ReturnDescriptor};
use itertools::join;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldSymRef {
    class_name: ClassName,
    name: FieldName,
    descriptor: TypeDescriptor,
}


impl FieldSymRef {
    pub fn new(descriptor: TypeDescriptor, class_name: ClassName, name: FieldName,) -> Self {
        FieldSymRef { class_name, name, descriptor }
    }

    pub fn class_name(&self) -> &ClassName {
        &self.class_name
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn descriptor(&self) -> &TypeDescriptor {
        &self.descriptor
    }
}


impl fmt::Display for FieldSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.descriptor, self.class_name, self.name)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodSymRef {
    return_desc: ReturnDescriptor,
    class_name: ClassName,
    name: MethodName,
    params_desc: Vec<TypeDescriptor>
}


impl MethodSymRef {
    pub fn new(
        return_desc: ReturnDescriptor,
        class_name: ClassName,
        name: MethodName,
        params_desc: Vec<TypeDescriptor>
    ) -> Self {
        MethodSymRef {
            return_desc,
            class_name,
            name,
            params_desc
        }
    }

    pub fn class_name(&self) -> &ClassName {
        &self.class_name
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
}


impl fmt::Display for MethodSymRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}({})", self.return_desc, self.class_name, self.name, join(&self.params_desc, ", "))
    }
}