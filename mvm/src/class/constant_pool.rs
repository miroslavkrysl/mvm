use crate::types::{Int, Long, Float, Double, ConstantValue};
use crate::class::{ClassSymRef, FieldSymRef, MethodSymRef, ConstantPoolError};
use std::convert::{TryFrom, TryInto};
use std::slice;


#[derive(Debug, Clone)]
pub struct ConstantPool {
    constants: Vec<ConstantValue>
}

impl ConstantPool {
    pub fn with_capacity(capacity: usize) -> Self {
        ConstantPool {
            constants: Vec::with_capacity(capacity)
        }
    }

    pub fn load<'a, T>(&'a self, index: usize) -> Result<&'a T, ConstantPoolError>
        where &'a T: TryFrom<&'a ConstantValue>,
              ConstantPoolError: From<<&'a T as TryFrom<&'a ConstantValue>>::Error> {
        let comp_value = self.load_constant(index)?;
        let value: &'a T = comp_value.try_into()?;
        Ok(value)
    }

    pub fn load_constant(&self, index: usize) -> Result<&ConstantValue, ConstantPoolError> {
        match self.constants.get(index) {
            None => Err(ConstantPoolError::IndexOutOfBounds),
            Some(constant) => {
                Ok(constant)
            }
        }
    }

    pub fn push<T: Into<ConstantValue>>(&mut self, value: T) {
        self.push_constant(value.into());
    }

    pub fn push_constant(&mut self, value: ConstantValue) {
        self.constants.push(value);
    }

    pub fn iter(&self) -> ConstantPoolIter {
        ConstantPoolIter {
            inner: self.constants.iter()
        }
    }
}


#[derive(Debug, Clone)]
pub struct ConstantPoolIter<'a> {
    inner: slice::Iter<'a, ConstantValue>
}

impl<'a> Iterator for ConstantPoolIter<'a> {
    type Item = &'a ConstantValue;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> ExactSizeIterator for ConstantPoolIter<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}