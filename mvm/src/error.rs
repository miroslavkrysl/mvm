use std::fmt;
use std::error::Error;
use crate::stack::error::{OperandStackError, VariablesError};

/// Unrecoverable VM error that can happen during the VM execution.
#[derive(Debug)]
pub enum VmError {
    OperandStack(OperandStackError),
    Variables(VariablesError),
}
