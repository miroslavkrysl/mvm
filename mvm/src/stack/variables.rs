use crate::types::{Operand, OperandSize};
use crate::stack::error::VariablesError;
use std::collections::BTreeMap;
use std::collections::btree_map;


#[derive(Debug, Clone)]
pub struct Variables {
    data: BTreeMap<usize, Operand>,
    size: usize,
}

impl Variables {
    pub fn new(size: usize) -> Self {
        Variables {
            data: BTreeMap::new(),
            size,
        }
    }

    pub fn store(&mut self, index: usize, operand: Operand) -> Result<(), VariablesError> {
        self.check_index_bounds(index, operand.size())?;

        if index != 0 {
            // invalidate previous 2nd category variable
            if let btree_map::Entry::Occupied(entry) = self.data.entry(index - 1) {
                if entry.get().size() == OperandSize::Double {
                    entry.remove();
                }
            }
        }

        if operand.size() == OperandSize::Double {
            // invalidate next variable
            if let btree_map::Entry::Occupied(entry) = self.data.entry(index + 1) {
                entry.remove();
            }
        }

        self.data.insert(index, operand);
        Ok(())
    }

    pub fn load(&mut self, index: usize) -> Result<Operand, VariablesError> {
        self.check_index_bounds(index, OperandSize::Single)?;

        match self.data.get(&index) {
            None => Err(VariablesError::Undefined),
            Some(&value) => Ok(value),
        }
    }

    fn check_index_bounds(&self, index: usize, operand_size: OperandSize) -> Result<(), VariablesError> {
        let reserve: usize = operand_size.into();

        if self.size >= reserve && self.size - reserve >= index {
            Ok(())
        } else {
            Err(VariablesError::IndexOutOfBounds)
        }
    }
}