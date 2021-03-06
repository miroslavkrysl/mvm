use std::io;

use thiserror::Error;

use crate::vm::class::descriptor::ReturnDesc;
use crate::vm::class::error::{ClassError, CodeError};
use crate::vm::class::name::ClassName;
use crate::vm::memory::error::{FrameError, LocalsError, OperandStackError};
use crate::vm::parse::error::{CreateClassError, ParseClassError};
use crate::vm::types::error::ValueError;
use crate::vm::types::value::ValueType;


#[derive(Error, Debug)]
pub enum ExecError {
    #[error("main method not found in class {class_name}")]
    MainMethodNotFound {
        class_name: ClassName
    },
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
    #[error(transparent)]
    Class {
        #[from]
        source: ClassError
    },
    #[error(transparent)]
    ClassLoad {
        #[from]
        source: ClassLoadError
    },
    #[error(transparent)]
    Code {
        #[from]
        source: CodeError
    },
    #[error(transparent)]
    Frame {
        #[from]
        source: FrameError
    },
    #[error(transparent)]
    Value {
        #[from]
        source: ValueError
    },
    #[error("invalid return call, expected return of {expected}, called {called}")]
    InvalidReturnType {
        expected: ReturnDesc,
        called: ValueType,
    },
}


#[derive(Error, Debug)]
#[error("can not load class {name}: {kind}")]
pub struct ClassLoadError {
    name: ClassName,
    kind: ClassLoadErrorKind,
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


#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error("class has wrong name: {name}")]
    WrongName {
        name: ClassName
    },
    #[error(transparent)]
    Parse {
        #[from]
        source: ParseClassError
    },
}