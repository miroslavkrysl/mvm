use std::convert::TryFrom;

use std::slice;
use crate::vm::types::value::{ValueCategory, Value};
use crate::vm::memory::error::LocalsError;
use std::sync::RwLock;


#[derive(Debug, Clone)]
pub enum Slot {
    Undefined,
    Value(Value),
}

#[derive(Debug)]
pub struct Locals {
    values: RwLock<Vec<Slot>>
}

impl Locals {
    /// Max allowed size of the locals array.
    pub const MAX_SIZE: usize = 255;
    
    pub fn new(size: usize) -> Self {
        Locals {
            values: RwLock::new(vec![Slot::Undefined; size])
        }
    }

    pub fn load<T>(&self, index: usize) -> Result<T, LocalsError>
        where T: TryFrom<Value>,
              LocalsError: From<<T as TryFrom<Value>>::Error> {
        let comp_value = self.load_value(index)?;
        let value = T::try_from(comp_value)?;
        Ok(value)
    }

    pub fn load_value(&self, index: usize) -> Result<Value, LocalsError> {
        let values = self.values.read().unwrap();
        match values.get(index) {
            None => Err(LocalsError::IndexOutOfBounds {
                index,
                size: values.len()
            }),
            Some(Slot::Undefined) => {
                Err(LocalsError::InvalidIndex)
            }
            Some(Slot::Value(value)) => {
                Ok(value.clone())
            }
        }
    }

    pub fn store<T: Into<Value>>(&self, index: usize, value: T) -> Result<(), LocalsError> {
        self.store_value(index, value.into())
    }

    pub fn store_value(&self, index: usize, value: Value) -> Result<(), LocalsError> {
        let mut values = self.values.write().unwrap();
        if index >= values.len() {
            return Err(LocalsError::IndexOutOfBounds {
                index,
                size: values.len()
            });
        }

        if index + value.value_type().category().size() > values.len() {
            return Err(LocalsError::InvalidIndex);
        }

        // check if the previous value is of double category and invalidate it eventually
        if index != 0 {
            if let Slot::Undefined = values[index] {
                if let Slot::Value(prev_value) = &values[index - 1] {
                    if prev_value.value_type().category() == ValueCategory::Double {
                        values[index - 1] = Slot::Undefined;
                    }
                }
            }
        }

        // check if the value is of double category and invalidate the next one eventually
        if value.value_type().category() == ValueCategory::Double {
            values[index + 1] = Slot::Undefined;
        }

        values[index] = Slot::Value(value);

        return Ok(());
    }
}


#[derive(Clone)]
pub enum LocalsEvent {
    Change(usize, Slot)
}

#[derive(Debug, Clone)]
pub struct LocalsIter<'a> {
    inner: slice::Iter<'a, Slot>
}


impl<'a> Iterator for LocalsIter<'a> {
    type Item = &'a Slot;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> ExactSizeIterator for LocalsIter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}
