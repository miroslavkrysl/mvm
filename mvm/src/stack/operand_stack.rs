use crate::types::{Int, Long, Float, Double, Reference, Operand, OperandSize};
use crate::stack::error::OperandStackError;


#[derive(Debug)]
pub struct OperandStack {
    data: Vec<Operand>,
    size: usize,
    capacity: usize,
}

impl OperandStack {
    pub fn new(capacity: usize) -> Self {
        OperandStack {
            data: Vec::with_capacity(capacity),
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, operand: Operand) -> Result<(), OperandStackError> {
        let operand_size: usize = operand.size().into();

        // check overflow
        if self.size + operand_size > self.capacity {
            return Err(OperandStackError::StackOverflow);
        }

        self.data.push(operand);
        self.size += operand_size;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<Operand, OperandStackError> {
        match self.data.pop() {
            None => Err(OperandStackError::StackUnderflow),
            Some(operand) => {
                self.size -= usize::from(operand.size());
                Ok(operand)
            },
        }
    }
}
