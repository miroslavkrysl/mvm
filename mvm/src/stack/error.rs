use std::fmt;
use std::error::Error;

/// An error caused by the inappropriate OperandStack manipulation.
#[derive(Debug)]
pub enum OperandStackError {
    StackOverflow,
    StackUnderflow,
    NonMatchingTypes,
}

impl fmt::Display for OperandStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandStackError::StackOverflow => write!(f, "stack overflow"),
            OperandStackError::StackUnderflow => write!(f, "stack underflow"),
            OperandStackError::NonMatchingTypes => write!(f, "non matching types"),
        }
    }
}

impl Error for OperandStackError {}