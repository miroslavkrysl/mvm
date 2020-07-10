use crate::instruction::Instruction;
use crate::class::error::CodeError;

#[derive(Debug, Clone)]
pub struct Code {
    locals_len: usize,
    instructions: Vec<Instruction>,
}

impl Code {
    pub fn new(locals_len: usize, instructions: Vec<Instruction>) -> Result<Self, CodeError> {
        if instructions.is_empty() {
            return Err(CodeError::NoInstructions)
        }

        Ok(Code { locals_len, instructions })
    }

    pub fn locals_len(&self) -> usize {
        self.locals_len
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn instruction(&self, index: usize) -> Result<&Instruction, CodeError> {
        match self.instructions.get(index) {
            None => Err(CodeError::IndexOutOfBounds { max: self.instructions.len(), index }),
            Some(instruction) => Ok(instruction),
        }
    }
}

