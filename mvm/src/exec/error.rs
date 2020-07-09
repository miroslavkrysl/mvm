use thiserror::Error;
use crate::memory::{OperandStackError, LocalsError};

#[derive(Error, Debug)]
pub enum ExecError {
    #[error(transparent)]
    OperandStack {
        #[from]
        source: OperandStackError
    },
    #[error(transparent)]
    Locals {
        #[from]
        source: LocalsError
    },
}