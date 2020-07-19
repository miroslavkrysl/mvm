use thiserror::Error;

use crate::vm::class::name::ClassName;
use crate::vm::types::error::ValueError;
use crate::vm::class::descriptor::TypeDesc;


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
    #[error("index {} is out of bounds, locals size is {size}")]
    IndexOutOfBounds {
        index: usize,
        size: usize,
    },
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
    #[error("expected argument of type {expected}")]
    IncompatibleArgumentType {
        expected: TypeDesc
    },
    #[error(transparent)]
    OperandStack {
        #[from]
        source: OperandStackError
    },
}