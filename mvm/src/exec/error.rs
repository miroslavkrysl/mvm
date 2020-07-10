use thiserror::Error;
use std::io;
use crate::memory::error::{OperandStackError, LocalsError};
use crate::class::name::ClassName;
use crate::class::error::ClassError;
use crate::parse::error::{ParseClassError, CreateClassError};


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

#[derive(Error, Debug)]
pub enum ClassLoadError {
    #[error("class {0} was not found")]
    ClassNotFound(ClassName),
    #[error("class {expected} found, but has wrong name: {got}")]
    WrongName {
        expected: ClassName,
        got: ClassName
    },
    #[error(transparent)]
    Parse {
        #[from]
        source: ParseClassError
    },
    #[error(transparent)]
    Create {
        #[from]
        source: CreateClassError
    },
    #[error(transparent)]
    Class {
        #[from]
        source: ClassError
    },
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error
    },
}