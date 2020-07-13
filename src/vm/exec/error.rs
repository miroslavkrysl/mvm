use thiserror::Error;
use std::io;
use crate::vm::memory::error::{OperandStackError, LocalsError};
use crate::vm::class::name::ClassName;
use crate::vm::class::error::ClassError;
use crate::vm::parse::error::{ParseClassError, CreateClassError};


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
#[error("can not load class {name}: {kind}")]
pub struct ClassLoadError {
    name: ClassName,
    kind: ClassLoadErrorKind
}


impl ClassLoadError {
    pub fn new(name: ClassName, kind: ClassLoadErrorKind) -> Self {
        ClassLoadError { name, kind }
    }
}


#[derive(Error, Debug)]
pub enum ClassLoadErrorKind {
    #[error("class was not found")]
    ClassNotFound,
    #[error("class has wrong name: {name}")]
    WrongName {
        name: ClassName
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
    Io {
        #[from]
        source: io::Error
    },
}