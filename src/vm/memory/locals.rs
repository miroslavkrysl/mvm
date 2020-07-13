use std::convert::TryFrom;

use std::slice;
use crate::vm::types::value::{CompValue, ValueCategory};
use crate::vm::memory::error::LocalsError;


#[derive(Debug, Clone, PartialEq)]
pub enum Slot {
    Undefined,
    Value(CompValue),
}

#[derive(Debug, Clone)]
pub struct Locals {
    values: Vec<Slot>,
}

impl Locals {
    /// Max allowed size of the locals array.
    pub const MAX_SIZE: usize = 255;
    
    pub fn new(size: usize) -> Self {
        Locals {
            values: vec![Slot::Undefined; size]
        }
    }

    pub fn load<T>(&mut self, index: usize) -> Result<T, LocalsError>
        where T: TryFrom<CompValue>,
              LocalsError: From<<T as TryFrom<CompValue>>::Error> {
        let comp_value = self.load_value(index)?;
        let value = T::try_from(comp_value)?;
        Ok(value)
    }

    pub fn load_value(&mut self, index: usize) -> Result<CompValue, LocalsError> {
        match self.values.get(index) {
            None => Err(LocalsError::IndexOutOfBounds {
                index,
                size: self.values.len()
            }),
            Some(Slot::Undefined) => {
                Err(LocalsError::InvalidIndex)
            }
            Some(Slot::Value(value)) => {
                Ok(value.clone())
            }
        }
    }

    pub fn store<T: Into<CompValue>>(&mut self, index: usize, value: T) -> Result<(), LocalsError> {
        self.store_value(index, value.into())
    }

    pub fn store_value(&mut self, index: usize, value: CompValue) -> Result<(), LocalsError> {
        if index >= self.values.len() {
            return Err(LocalsError::IndexOutOfBounds {
                index,
                size: self.values.len()
            });
        }

        if index + value.value_type().category().size() > self.values.len() {
            return Err(LocalsError::InvalidIndex);
        }

        // check if the previous value is of double category and invalidate it eventually
        if index != 0 {
            if let Slot::Undefined = self.values[index] {
                if let Slot::Value(prev_value) = &self.values[index - 1] {
                    if prev_value.value_type().category() == ValueCategory::Double {
                        self.values[index - 1] = Slot::Undefined;
                    }
                }
            }
        }

        // check if the value is of double category and invalidate the next one eventually
        if value.value_type().category() == ValueCategory::Double {
            self.values[index + 1] = Slot::Undefined;
        }

        self.values[index] = Slot::Value(value);
        return Ok(());
    }

    pub fn iter(&self) -> LocalsIter {
        LocalsIter {
            inner: self.values.iter()
        }
    }
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