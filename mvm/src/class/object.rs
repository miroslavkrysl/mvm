use std::iter;
use std::sync::{Arc, RwLock};

use crate::class::{ArrayError, ArrayValueDescriptor, Class, SimpleValueDescriptor};
use crate::types::{Boolean, Byte, Char, Double, Float, Int, JvmValue, Long, Reference, Short};

#[derive(Debug)]
pub enum Object {
    Instance(Instance),
    Array(Array),
}


#[derive(Debug)]
pub struct Instance {
    class: Arc<Class>,
    fields: RwLock<Vec<JvmValue>>,
}

impl Instance {
    pub fn new(class: Arc<Class>) -> Self {
        let fields = RwLock::new(
            class.fields()
                .filter(|field| !field.is_static())
                .map(|field| field.descriptor().value_desc().default_value())
                .collect());

        Instance {
            class,
            fields,
        }
    }

    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }
}

/// Field access.
impl Instance {
    /// Get the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn field(&self, index: usize) -> JvmValue {
        self.fields.read().unwrap()[index].clone()
    }

    /// Set the field value.
    ///
    /// # Panics
    ///
    /// Will panic if the index is out of bounds.
    pub fn set_field(&self, index: usize, value: JvmValue) {
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
    Boolean(RwLock<Box<[Boolean]>>),
    Char(RwLock<Box<[Char]>>),
    Reference(RwLock<Box<[Reference]>>),
}

impl Array {
    pub fn new(descriptor: ArrayValueDescriptor) -> Self {
        match descriptor.values_desc() {
            SimpleValueDescriptor::Byte =>
                Array::Byte(RwLock::new(
                    iter::repeat(Byte::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Char =>
                Array::Char(RwLock::new(
                    iter::repeat(Char::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Double =>
                Array::Double(RwLock::new(
                    iter::repeat(Double::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Float =>
                Array::Float(RwLock::new(
                    iter::repeat(Float::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Int =>
                Array::Int(RwLock::new(
                    iter::repeat(Int::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Long =>
                Array::Long(RwLock::new(
                    iter::repeat(Long::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Reference(_) =>
                Array::Reference(RwLock::new(
                    iter::repeat(Reference::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Short =>
                Array::Short(RwLock::new(
                    iter::repeat(Short::default()).take(*descriptor.dim().as_u8() as usize).collect())),
            SimpleValueDescriptor::Boolean =>
                Array::Boolean(RwLock::new(
                    iter::repeat(Boolean::default()).take(*descriptor.dim().as_u8() as usize).collect())),
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
            Array::Boolean(array) => array.read().unwrap().len(),
            Array::Char(array) => array.read().unwrap().len(),
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

    pub fn get_char(&self, index: usize) -> Result<Char, ArrayError> {
        match self {
            Array::Char(array) => {
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

    pub fn get_boolean(&self, index: usize) -> Result<Boolean, ArrayError> {
        match self {
            Array::Boolean(array) => {
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

    pub fn set_char(&self, index: usize, value: Char) -> Result<(), ArrayError> {
        match self {
            Array::Char(array) => {
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

    pub fn set_boolean(&self, index: usize, value: Boolean) -> Result<(), ArrayError> {
        match self {
            Array::Boolean(array) => {
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