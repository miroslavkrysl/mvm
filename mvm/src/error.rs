use std::fmt;
use std::error::Error;

/// Unrecoverable VM error that can happen during the VM execution.
#[derive(Debug)]
pub enum VmError {
    OperandStackError(OperandStackError),
    LocalsError(LocalsError),
}


/// An error caused by the inappropriate OperandStack manipulation.
#[derive(Debug)]
pub enum OperandStackError {
    StackOverflow,
    StackUnderflow,
}

impl fmt::Display for OperandStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandStackError::StackOverflow => write!(f, "stack overflow"),
            OperandStackError::StackUnderflow => write!(f, "stack underflow"),
        }
    }
}

impl Error for OperandStackError {}


/// An error caused by the inappropriate Locals manipulation.
#[derive(Debug)]
pub enum LocalsError {
    IndexOutOfBounds
}

impl fmt::Display for LocalsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalsError::IndexOutOfBounds => write!(f, "index out of bounds"),
        }
    }
}

impl Error for LocalsError {}