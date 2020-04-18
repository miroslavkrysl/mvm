//! VM stack area structs.

use crate::types::{Word, DoubleWord};
use crate::error::{OperandStackError, LocalsError};

/// A LIFO struct for storing operands of type Word or DoubleWord.
/// DoubleWord is split into two Words and stored in big endian order.
/// The maximal capacity must be provided on initialization.
/// Maximal value of capacity is equal to the maximum value of the 16 bit unsigned integer.
pub struct OperandStack {
    data: Vec<Word>,
    capacity: usize,
}

impl OperandStack {
    pub fn new(capacity: u16) -> Self {
        OperandStack {
            data: Vec::with_capacity(capacity as usize),
            capacity: capacity as usize,
        }
    }

    pub fn push_word(&mut self, value: Word) -> Result<(), OperandStackError> {
        if self.data.len() >= self.capacity {
            return Err(OperandStackError::StackOverflow);
        }

        self.data.push(value);
        Ok(())
    }

    pub fn push_double_word(&mut self, value: DoubleWord) -> Result<(), OperandStackError> {
        let words = value.to_words_be();
        self.push_word(words[0])?;
        self.push_word(words[1])?;

        Ok(())
    }

    pub fn pop_word(&mut self) -> Result<Word, OperandStackError> {
        self.data.pop().ok_or(OperandStackError::StackUnderflow)
    }

    pub fn pop_double_word(&mut self) -> Result<DoubleWord, OperandStackError> {
        let h = self.pop_word()?;
        let l = self.pop_word()?;

        Ok(DoubleWord::from_words_be(&[h, l]))
    }
}

/// An array-like struct for storing local variables of type Word or DoubleWord.
/// DoubleWord is split into two Words and stored in big endian order.
/// The size of the Locals array must be provided on initialization.
/// Maximal value of the size of the Locals array is equal to the maximum
/// value of the 16 bit unsigned integer.
pub struct Locals {
    data: Vec<Word>
}

impl Locals {
    pub fn new(size: u16) -> Self {
        Locals {
            data: vec![Word::new(0); size as usize]
        }
    }

    pub fn store_word(&mut self, index: u16, value: Word) -> Result<(), LocalsError> {
        let mut w = self.data.get_mut(index as usize)
            .ok_or(LocalsError::IndexOutOfBounds)?;

        *w = value;
        Ok(())
    }

    pub fn store_double_word(&mut self, index: u16, value: DoubleWord) -> Result<(), LocalsError> {
        let words = value.to_words_be();

        {
            let h = self.data.get_mut(index as usize)
                .ok_or(LocalsError::IndexOutOfBounds)?;
            *h = words[0];
        }
        {
            let l = self.data.get_mut((index as usize) + 1)
                .ok_or(LocalsError::IndexOutOfBounds)?;
            *l = words[1];
        }

        Ok(())
    }

    pub fn load_word(&self, index: u16) -> Result<Word, LocalsError> {
        self.data.get(index as usize)
            .cloned()
            .ok_or(LocalsError::IndexOutOfBounds)
    }

    pub fn load_double_word(&self, index: u16) -> Result<DoubleWord, LocalsError> {
        let h = self.data.get(index as usize)
            .cloned()
            .ok_or(LocalsError::IndexOutOfBounds)?;
        let l = self.data.get((index as usize) + 1)
            .cloned()
            .ok_or(LocalsError::IndexOutOfBounds)?;

        Ok(DoubleWord::from_words_be(&[h, l]))
    }
}
