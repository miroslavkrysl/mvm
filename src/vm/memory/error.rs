use thiserror::Error;

use crate::vm::class::name::ClassName;
use crate::vm::types::error::ValueError;
use crate::vm::class::descriptor::TypeDesc;
use crate::vm::types::value::ValueType;


/// An error caused by the inappropriate OperandStack manipulation.
#[derive(Error, Debug)]
pub enum OperandStackError {
    #[error("stack overflow")]
    Overflow,
    #[error("stack underflow")]
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
    #[error("expected argument of type {expected}, got {}")]
    IncompatibleArgumentType {
        expected: TypeDesc,
        got: ValueType
    },
    #[error(transparent)]
    OperandStack {
        #[from]
        source: OperandStackError
    },
}