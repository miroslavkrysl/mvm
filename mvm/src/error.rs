use thiserror::Error;
use crate::memory::{OperandStackError, LocalsError};

/// Unrecoverable VM error that can happen during the VM execution.
#[derive(Debug)]
pub enum VmError {
    OperandStack(OperandStackError),
    Locals(LocalsError),
}
