use std::sync::Arc;

use crate::class::class::Class;
use crate::types::value::Value;


/// A class instance object with `usize` indexable fields.
/// Newly created is marked uninitialized, can be marked initialized.
#[derive(Debug, Clone)]
pub struct Instance {
    class: Arc<Class>,
    is_initialized: bool,
    fields: Vec<Value>,
}


impl Instance {
    pub fn new(class: Arc<Class>) -> Self {
        let fields = class.fields()
                          .filter(|field| !field.is_static())
                          .map(|field|
                              field.signature()
                                   .type_desc()
                                   .value_type()
                                   .default_value())
                          .collect();

        Instance {
            class,
            is_initialized: false,
            fields,
        }
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn set_initialized(&mut self) {
        self.is_initialized = true;
    }
}


/// Field access.
impl Instance {
    /// Get the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn field(&self, index: usize) -> Value {
        self.fields[index].clone()
    }

    /// Set the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn set_field(&mut self, index: usize, value: Value) {
        self.fields[index] = value
    }
}
