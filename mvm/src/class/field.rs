use crate::class::{FieldFlags, FieldDescriptor, FieldName};
use crate::types::{Int, Float, Long, Double};

#[derive(Debug, Clone)]
pub struct Field {
    name: FieldName,
    flags: FieldFlags,
    descriptor: FieldDescriptor,
    constant_value: Option<FieldConst>
}

impl Field {
    pub fn new(name: FieldName, flags: FieldFlags, descriptor: FieldDescriptor, constant_value: Option<FieldConst>) -> Self {
        Field {
            name,
            flags,
            descriptor,
            constant_value
        }
    }

    pub fn flags(&self) -> &FieldFlags {
        &self.flags
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn descriptor(&self) -> &FieldDescriptor {
        &self.descriptor
    }
}

#[derive(Debug, Clone)]
pub enum FieldConst {
    Int(Int),
    Float(Float),
    Long(Long),
    Double(Double),
    // String
}
