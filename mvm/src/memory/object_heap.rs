
use crate::class::{Class, ArrayValueDescriptor, Instance, Array};
use crate::types::Reference;
use std::sync::Arc;
use std::collections::{LinkedList, HashMap};
use crate::memory::Object;
use crate::memory::layout::Layout;
use std::hash::Hash;


pub struct ObjectHeap {
    objects: Vec<Arc<Object>>,
}

impl ObjectHeap {
    pub fn create_instance(&self, class: Arc<Class>) -> Reference {

        self.objects.
    }

    pub fn create_array(&self, descriptor: ArrayValueDescriptor) -> Reference {
        let array
        self.objects.
    }
}