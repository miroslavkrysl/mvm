use std::fmt::Display;
use std::sync::{Arc, Mutex, RwLock};

use crate::vm::class::class::Class;
use crate::vm::types::value::Value;
use crate::vm::class::field::Field;


/// A class instance pointer - wrapper around the data pointer.
#[derive(Debug, Clone)]
pub struct InstancePtr {
    data: Arc<InstanceData>
}


/// A class instance data.
#[derive(Debug)]
struct InstanceData {
    class: Arc<Class>,
    fields: Mutex<Vec<Value>>,
}


impl InstancePtr {
    pub fn new(class: Arc<Class>) -> Self {
        let fields = class.fields()
                          .filter(|field| !field.is_static())
                          .map(|field|
                              field.signature()
                                   .type_desc()
                                   .value_type()
                                   .default_value())
                          .collect();

        InstancePtr {
            data: Arc::new(InstanceData {
                class,
                fields: Mutex::new(fields)
            })
        }
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.data.class
    }

    /// Get the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn field(&self, index: usize) -> Value {
        self.data.fields.lock().unwrap()[index].clone()
    }

    /// Set the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn set_field(&self, index: usize, value: Value) {
        self.data.fields.lock().unwrap()[index] = value
    }

    /// Get the instance id.
    pub fn id(&self) -> InstanceId {
        InstanceId(Arc::as_ptr(&self.data) as usize)
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct InstanceId(usize);


impl Display for InstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}