use thiserror::Error;
use crate::vm::types::value::ValueType;

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("expected value type {expected} but found {found}")]
    TypeMismatch {
        expected: ValueType,
        found: ValueType
    },
    #[error("reference is null")]
    NullPointer,
    #[error("division by zero")]
    DivisionByZero
}