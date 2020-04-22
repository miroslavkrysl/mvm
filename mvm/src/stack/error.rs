use std::fmt;
use std::error::Error;

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


/// An error caused by the inappropriate Variables manipulation.
#[derive(Debug)]
pub enum VariablesError {
    IndexOutOfBounds,
    Undefined,
}

impl fmt::Display for VariablesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariablesError::IndexOutOfBounds => write!(f, "index out of bounds"),
            VariablesError::Undefined => write!(f, "variable value is undefined"),
        }
    }
}

impl Error for VariablesError {}