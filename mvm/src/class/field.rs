use crate::class::name::FieldName;
use crate::class::flags::FieldFlags;
use crate::class::descriptor::TypeDescriptor;
use crate::types::jvm_value::JvmValue;
use crate::types::int::Int;
use crate::types::float::Float;
use crate::types::long::Long;
use crate::types::double::Double;


#[derive(Debug, Clone)]
pub struct Field {
    name: FieldName,
    flags: FieldFlags,
    descriptor: TypeDescriptor,
    constant_value: Option<FieldConst>,
}

impl Field {
    pub fn new(name: FieldName, flags: FieldFlags, descriptor: TypeDescriptor, constant_value: Option<FieldConst>) -> Self {
        Field {
            name,
            flags,
            descriptor,
            constant_value,
        }
    }
}

/// Getters
impl Field {
    pub fn flags(&self) -> &FieldFlags {
        &self.flags
    }

    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn descriptor(&self) -> &TypeDescriptor {
        &self.descriptor
    }

    pub fn constant_value(&self) -> Option<FieldConst> {
        self.constant_value
    }
}

impl Field {
    pub fn init_value(&self) -> JvmValue {
        match self.constant_value {
            None => self.descriptor.default_value(),
            Some(value) => {
                value.into()
            },
        }
    }
}

/// Field access and properties related logic.
impl Field {
    pub fn is_static(&self) -> bool {
        self.flags.is_static()
    }
}


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