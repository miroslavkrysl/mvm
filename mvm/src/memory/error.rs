use thiserror::Error;
use crate::types::ValueError;

/// An error caused by the inappropriate OperandStack manipulation.
#[derive(Error, Debug)]
pub enum OperandStackError {
    #[error("stack underflow")]
    Overflow,
    #[error("stack overflow")]
    Underflow,
    #[error("unsupported type for operation")]
    InvalidType,
    #[error(transparent)]
    Value {
        #[from]
        source: ValueError
    },
}


/// An error caused by the inappropriate Variables manipulation.
#[derive(Error, Debug)]
pub enum VariablesError {
    #[error("index out of bounds")]
    IndexOutOfBounds,
}
