
use std::sync::Arc;
use crate::class::class::Class;
use crate::class::object::Object;
use crate::memory::error::HeapError;
use crate::types::reference::Reference;
use crate::class::name::ClassName;
use crate::class::descriptor::ArrayDescriptor;


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
    pub fn class(class_name: &ClassName) -> Result<Arc<Class>, HeapError> {
        unimplemented!()
    }

    pub fn add_class(class: Arc<Class>) -> Result<(), HeapError> {
        unimplemented!()
    }

    pub fn create_instance(class: Arc<Class>) -> Result<Reference, HeapError> {
        unimplemented!()
    }

    pub fn create_array(descriptor: ArrayDescriptor) -> Result<Reference, HeapError> {
        unimplemented!()
    }
}