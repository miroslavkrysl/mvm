
use std::sync::{Arc, RwLock};
use crate::vm::class::class::Class;
use crate::vm::memory::error::HeapError;
use crate::vm::class::name::ClassName;
use crate::vm::class::instance::{InstancePtr};
use std::collections::HashMap;



pub struct Heap {
    classes: RwLock<HashMap<ClassName, Arc<Class>>>,
    instances: RwLock<Vec<Arc<InstancePtr>>>,
}


impl Heap {
    pub fn new() -> Self {
        Heap {
            classes: RwLock::new(HashMap::new()),
            instances: RwLock::new(Vec::new())
        }
    }
}
impl Heap {
    pub fn class(&self, class_name: &ClassName) -> Option<Arc<Class>> {
        self.classes.read().unwrap().get(class_name).cloned()
    }

    pub fn add_class(&self, class: Class) -> Arc<Class> {
        let class = Arc::new(class);
        self.classes.write().unwrap().insert(class.name().clone(), class.clone());
        class
    }

    pub fn add_instance(&self, instance: InstancePtr) -> Arc<InstancePtr> {
        let instance = Arc::new(instance);
        self.instances.write().unwrap().push(instance.clone());
        instance
    }
}