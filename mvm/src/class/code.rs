use crate::instruction::Instruction;
use crate::class::error::CodeError;

#[derive(Debug, Clone)]
pub struct Code {
    max_stack: usize,
    max_locals: usize,
    instructions: Vec<Instruction>,
}

impl Code {
    pub fn new(max_stack: usize, max_locals: usize, instructions: Vec<Instruction>) -> Self {
        Code { max_stack, max_locals, instructions }
    }

    pub fn max_stack(&self) -> usize {
        self.max_stack
    }

    pub fn max_locals(&self) -> usize {
        self.max_locals
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn instruction(&self, index: usize) -> Result<&Instruction, CodeError> {
        match self.instructions.get(index) {
            None => Err(CodeError::IndexOutOfBounds),
            Some(instruction) => Ok(instruction),
        }
    }
}

