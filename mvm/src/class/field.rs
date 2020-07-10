use crate::class::name::FieldName;
use crate::class::descriptor::TypeDescriptor;
use crate::types::jvm_value::JvmValue;
use crate::types::int::Int;
use crate::types::float::Float;
use crate::types::long::Long;
use crate::types::double::Double;


#[derive(Debug, Clone)]
pub struct Field {
    name: FieldName,
    is_static: bool,
    descriptor: TypeDescriptor
}

impl Field {
    pub fn new(descriptor: TypeDescriptor, name: FieldName, is_static: bool) -> Self {
        Field {
            name,
            is_static,
            descriptor
        }
    }
}

/// Getters
impl Field {
    pub fn name(&self) -> &FieldName {
        &self.name
    }

    pub fn descriptor(&self) -> &TypeDescriptor {
        &self.descriptor
    }

    pub fn is_static(&self) -> bool {
        self.is_static
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