use std::fmt;
use std::error::Error;

/// Unrecoverable VM error that can happen during the VM execution.
#[derive(Debug)]
pub enum VmError {
    OperandStackError(OperandStackError),
    LocalsError(LocalsError),
}


/// An error caused by the inappropriate Locals manipulation.
#[derive(Debug)]
pub enum LocalsError {
    IndexOutOfBounds,
    NonMatchingType,
    VariableOccupied,
    VariableEmpty,
}

impl fmt::Display for LocalsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalsError::IndexOutOfBounds => write!(f, "index out of bounds"),
            LocalsError::NonMatchingType => write!(f, "non matching type"),
            LocalsError::VariableOccupied => write!(f, "variable is occupied"),
            LocalsError::VariableEmpty => write!(f, "variable is empty")
        }
    }
}

impl Error for LocalsError {}