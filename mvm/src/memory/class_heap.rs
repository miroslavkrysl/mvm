use std::collections::HashMap;
use std::sync::Arc;
use crate::class::{Class, ClassName, FieldSymRef, ClassSymRef, ArrayValueDescriptor};
use crate::types::{JvmValue, CompValue, Reference};
use std::convert::TryFrom;
use crate::memory::HeapError;
use crate::memory::class_loader::ClassLoader;

pub struct Heap {
    class_loader: ClassLoader,
    classes: HashMap<ClassName, Arc<Class>>,
    objects: Vec<Arc<Class>>
}

pub trait ObjectHeap {
}

impl Heap {
    pub fn new(class_loader: ClassLoader) -> Self {
        Heap {
            class_loader,
            classes: HashMap::new(),
            objects: Vec::new()
        }
    }

    pub fn create_instance(&self, class: Arc<Class>) -> Reference {
        let instance = Arc::new(class.create_instance());
        let 
    }

    pub fn create_array(&self, descriptor: ArrayValueDescriptor) -> Reference {
        unimplemented!()
    }

    pub fn class(&self, sym_ref: ClassSymRef) -> Result<&Arc<Class>, HeapError> {
        unimplemented!()
    }
}