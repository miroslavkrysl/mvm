use crate::class::{FieldFlags, FieldDescriptor, FieldName, Class};
use crate::types::{Int, Float, Long, Double, JvmValue};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Field {
    class: Arc<Class>,
    name: FieldName,
    flags: FieldFlags,
    descriptor: FieldDescriptor,
    constant_value: Option<FieldConst>,
}

impl Field {
    pub fn new(class: Arc<Class>, name: FieldName, flags: FieldFlags, descriptor: FieldDescriptor, constant_value: Option<FieldConst>) -> Self {
        Field {
            class,
            name,
            flags,
            descriptor,
            constant_value,
        }
    }
}

/// Getters
impl Field {
    pub fn class(&self) -> &Arc<Class> {
        &self.class
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

    pub fn constant_value(&self) -> Option<FieldConst> {
        self.constant_value
    }
}

impl Field {
    pub fn init_value(&self) -> JvmValue {
        match self.constant_value {
            None => self.descriptor.value_desc().default_value(),
            Some(value) => {
                value.into()
            },
        }
    }
}

/// Field access and properties related logic.
impl Field {
    pub fn is_static(&self) -> bool {
        self.flags.has(FieldFlags::STATIC)
    }

    pub fn is_final(&self) -> bool {
        self.flags.has(FieldFlags::FINAL)
    }

    pub fn is_accessible_for(&self, class: &Class) -> bool {
        // TODO: implement inter-class accessibility
        true
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.descriptor == other.descriptor && self.class == other.class
    }
}

impl Eq for Field {}


#[derive(Debug, Copy, Clone)]
pub enum FieldConst {
    Int(Int),
    Float(Float),
    Long(Long),
    Double(Double),
}

impl From<FieldConst> for JvmValue {
    fn from(field_const: FieldConst) -> Self {
        match field_const {
            FieldConst::Int(value) => value.into(),
            FieldConst::Float(value) => value.into(),
            FieldConst::Long(value) => value.into(),
            FieldConst::Double(value) => value.into(),
        }
    }
}