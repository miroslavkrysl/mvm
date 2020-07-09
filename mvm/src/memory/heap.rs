use crate::class::{ClassSymRef, MethodSymRef, FieldSymRef, ValueDescriptor, ArrayValueDescriptor, Class, Object};
use crate::types::{Reference, JvmValue};
use crate::memory::{HeapError};
use std::collections::HashMap;
use std::sync::Arc;

pub struct Heap {
    classes: Vec<Arc<Class>>,
    objects: Vec<Arc<Object>>,
}


impl Heap {
    pub fn new() -> Self {
        Heap {
            classes: Vec::new(),
            objects: Vec::new()
        }
    }
}
impl Heap {
    pub fn class(sym_ref: &ClassSymRef) -> Result<Arc<Class>, HeapError> {
        unimplemented!()
    }

    pub fn add_class(class: Arc<Class>) -> Result<(), HeapError> {
        unimplemented!()
    }

    pub fn create_instance(class: Arc<Class>) -> Result<Reference, HeapError> {
        unimplemented!()
    }

    pub fn create_array(descriptor: ArrayValueDescriptor) -> Result<Reference, HeapError> {
        unimplemented!()
    }
}