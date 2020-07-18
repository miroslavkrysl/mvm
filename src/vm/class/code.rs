use crate::vm::class::error::CodeError;
use crate::vm::memory::locals::Locals;
use crate::vm::bytecode::instruction::Instruction;


/// A method code.
/// It is an indexable array of bytecode instructions together with
/// predefined size of locals array needed for execution.
#[derive(Debug, Clone)]
pub struct Code {
    locals_size: usize,
    instructions: Vec<Instruction>,
}


impl Code {
    /// Create a new `Code` with the given locals array size and instruction.
    ///
    /// # Errors
    ///
    /// Returns `CodeError::NoInstructions` if no instructions are given
    /// or `CodeError::TooBigLocalsSize` if `locals_size` is bigger than allowed.
    pub fn new(locals_size: usize, instructions: Vec<Instruction>) -> Result<Self, CodeError> {
        if instructions.is_empty() {
            return Err(CodeError::NoInstructions);
        }

        if locals_size > Locals::MAX_SIZE {
            return Err(CodeError::TooBigLocalsSize {
                size: locals_size,
                max: Locals::MAX_SIZE,
            });
        }

        Ok(Code { locals_size, instructions })
    }

    /// Get the locals size.
    pub fn locals_size(&self) -> usize {
        self.locals_size
    }

    /// Get the instructions.
    pub fn instructions(&self) -> impl ExactSizeIterator<Item=&Instruction> {
        self.instructions.iter()
    }

    /// Get the instruction on the given index.
    ///
    /// # Errors
    ///
    /// Returns `CodeError::InstructionOutOfBounds` if the index is greater than the last instruction index.
    pub fn instruction(&self, index: isize) -> Result<Instruction, CodeError> {
        if index < 0 {
            return Err(CodeError::IndexOutOfBounds { max: self.instructions.len(), index });
        }

        match self.instructions.get(index as usize) {
            None => Err(CodeError::IndexOutOfBounds { max: self.instructions.len(), index }),
            Some(instruction) => Ok(instruction.clone()),
        }
    }
}

