use thiserror::Error;
use crate::vm::types::value::ValueType;


#[derive(Error, Debug)]
#[error("division by zero")]
pub struct DivisionByZero;

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("expected value type {expected} but found {found}")]
    TypeMismatch {
        expected: ValueType,
        found: ValueType
    },
    #[error("unexpected value category")]
    UnexpectedCategory,
    #[error("reference is null")]
    NullPointer
}