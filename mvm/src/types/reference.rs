use std::fmt;
use std::intrinsics::transmute;
use std::sync::Arc;

use crate::class::object::Object;
use crate::types::category::{ValueCategory, Categorize, Describe};
use crate::class::descriptor::TypeDescriptor;


#[derive(Debug, Clone)]
pub enum Reference {
    Null,
    Object(Arc<Object>),
}

impl Reference {
    pub fn null() -> Self {
        Reference::Null
    }

    pub fn new(ptr: Arc<Object>) -> Self {
        Reference::Object(ptr)
    }
}

impl Categorize for Reference {
    fn category() -> ValueCategory {
        ValueCategory::Single
    }
}

impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Reference::Null, Reference::Null) => true,
            (Reference::Object(ptr1), Reference::Object(ptr2)) => Arc::ptr_eq(ptr1, ptr2),
            _ => false
        }
    }
}

impl Eq for Reference {}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::Null => write!(f, "null"),
            Reference::Object(object) => {
                let ptr = unsafe { transmute::<_, usize>(object) };

                match object.as_ref() {
                    Object::Instance(instance) => write!(f, "{}@{}", instance.class().name(), ptr),
                    Object::Array(array) => unimplemented!(),
                }
            }
        }
    }
}

impl Default for Reference {
    fn default() -> Self {
        Self::null()
    }
}

impl Describe for Reference {
    fn descriptor() -> TypeDescriptor {
        unimplemented!()
    }
}