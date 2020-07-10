use thiserror::Error;
use crate::types::error::ValueError;


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
pub enum LocalsError {
    #[error("index out of bounds")]
    IndexOutOfBounds,
    #[error("locals was accessed on invalid index")]
    InvalidIndex,
    #[error(transparent)]
    Value {
        #[from]
        source: ValueError
    },
}


#[derive(Error, Debug)]
pub enum FrameError {
    #[error("incompatible arguments")]
    IncompatibleArguments
}


#[derive(Error, Debug)]
pub enum HeapError {
    
}