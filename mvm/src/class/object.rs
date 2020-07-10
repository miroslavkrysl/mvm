use std::iter;
use std::sync::{Arc, RwLock};
use crate::class::class::Class;
use crate::types::value::Value;
use crate::types::byte::Byte;
use crate::types::short::Short;
use crate::types::int::Int;
use crate::types::long::Long;
use crate::types::float::Float;
use crate::types::double::Double;
use crate::types::reference::Reference;
use crate::class::descriptor::SimpleDescriptor;
use crate::class::error::ArrayError;


#[derive(Debug)]
pub enum Object {
    Instance(Instance),
    Array(Array),
}


#[derive(Debug)]
pub struct Instance {
    class: Arc<Class>,
    is_initialized: bool,
    fields: RwLock<Vec<Value>>,
}

impl Instance {
    pub fn new(class: Arc<Class>) -> Self {
        let fields = RwLock::new(
            class.fields()
                .filter(|field| !field.is_static())
                .map(|field|
                    field.signature()
                        .type_desc()
                        .default_value())
                .collect());

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
}

/// Field access.
impl Instance {
    /// Get the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn field(&self, index: usize) -> Value {
        self.fields.read().unwrap()[index].clone()
    }

    /// Set the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn set_field(&self, index: usize, value: Value) {
        self.fields.write().unwrap()[index] = value
    }
}


#[derive(Debug)]
pub enum Array {
    Byte(RwLock<Box<[Byte]>>),
    Short(RwLock<Box<[Short]>>),
    Int(RwLock<Box<[Int]>>),
    Long(RwLock<Box<[Long]>>),
    Float(RwLock<Box<[Float]>>),
    Double(RwLock<Box<[Double]>>),
    Reference(RwLock<Box<[Reference]>>),
}

impl Array {
    /// Max allowed number of array dimensions.
    pub const MAX_DIM: usize = 255;

    pub fn new(descriptor: SimpleDescriptor, length: usize) -> Self {
        match descriptor {
            SimpleDescriptor::Byte =>
                Array::Byte(RwLock::new(
                    iter::repeat(Byte::default()).take(length).collect())),
            SimpleDescriptor::Double =>
                Array::Double(RwLock::new(
                    iter::repeat(Double::default()).take(length).collect())),
            SimpleDescriptor::Float =>
                Array::Float(RwLock::new(
                    iter::repeat(Float::default()).take(length).collect())),
            SimpleDescriptor::Int =>
                Array::Int(RwLock::new(
                    iter::repeat(Int::default()).take(length).collect())),
            SimpleDescriptor::Long =>
                Array::Long(RwLock::new(
                    iter::repeat(Long::default()).take(length).collect())),
            SimpleDescriptor::Reference(_) =>
                Array::Reference(RwLock::new(
                    iter::repeat(Reference::default()).take(length).collect())),
            SimpleDescriptor::Short =>
                Array::Short(RwLock::new(
                    iter::repeat(Short::default()).take(length).collect())),
        }
    }

    pub fn length(&self) -> usize {
        match self {
            Array::Byte(array) => array.read().unwrap().len(),
            Array::Short(array) => array.read().unwrap().len(),
            Array::Int(array) => array.read().unwrap().len(),
            Array::Long(array) => array.read().unwrap().len(),
            Array::Float(array) => array.read().unwrap().len(),
            Array::Double(array) => array.read().unwrap().len(),
            Array::Reference(array) => array.read().unwrap().len(),
        }
    }

    pub fn get_byte(&self, index: usize) -> Result<Byte, ArrayError> {
        match self {
            Array::Byte(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_double(&self, index: usize) -> Result<Double, ArrayError> {
        match self {
            Array::Double(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_float(&self, index: usize) -> Result<Float, ArrayError> {
        match self {
            Array::Float(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_int(&self, index: usize) -> Result<Int, ArrayError> {
        match self {
            Array::Int(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_long(&self, index: usize) -> Result<Long, ArrayError> {
        match self {
            Array::Long(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_reference(&self, index: usize) -> Result<Reference, ArrayError> {
        match self {
            Array::Reference(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn get_short(&self, index: usize) -> Result<Short, ArrayError> {
        match self {
            Array::Short(array) => {
                array.read().unwrap().get(index).cloned()
                    .ok_or(ArrayError::IndexOutOfBounds)
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_byte(&mut self, index: usize, value: Byte) -> Result<(), ArrayError> {
        match self {
            Array::Byte(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_double(&self, index: usize, value: Double) -> Result<(), ArrayError> {
        match self {
            Array::Double(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_float(&self, index: usize, value: Float) -> Result<(), ArrayError> {
        match self {
            Array::Float(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_int(&self, index: usize, value: Int) -> Result<(), ArrayError> {
        match self {
            Array::Int(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_long(&self, index: usize, value: Long) -> Result<(), ArrayError> {
        match self {
            Array::Long(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_reference(&self, index: usize, value: Reference) -> Result<(), ArrayError> {
        match self {
            Array::Reference(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }

    pub fn set_short(&self, index: usize, value: Short) -> Result<(), ArrayError> {
        match self {
            Array::Short(array) => {
                match array.write().unwrap().get_mut(index) {
                    None => Err(ArrayError::IndexOutOfBounds),
                    Some(old_value) => {
                        *old_value = value;
                        Ok(())
                    }
                }
            }
            _ => Err(ArrayError::TypeMismatch)
        }
    }
}