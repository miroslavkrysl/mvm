use std::fmt;
use std::fmt::Display;
use std::num::{ParseIntError, ParseFloatError};

use thiserror::Error;

use crate::vm::class::error::{DescriptorError, NameError, ClassError, MethodError, CodeError, SignatureError};


#[derive(Error, Debug)]
pub struct ParseClassError {
    kind: ParseClassErrorKind,
    line_pos: usize,
}

impl ParseClassError {
    pub fn new(kind: ParseClassErrorKind, line_pos: usize) -> Self {
        ParseClassError {
            kind,
            line_pos
        }
    }
}

impl Display for ParseClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "on line {}: {}", self.line_pos, self.kind)
    }
}


#[derive(Error, Debug)]
pub enum ParseClassErrorKind {
    #[error("unknown entry: {0}")]
    UnknownEntry(String),
    #[error("unknown instruction: {0}")]
    UnknownInstruction(String),
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("unexpected end of line")]
    UnexpectedEndOfLine,
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("invalid method params descriptor: {0}")]
    InvalidParamsDescriptor(String),
    #[error("invalid field definition: {0}")]
    InvalidFieldDefinition(String),
    #[error("invalid method definition: {0}")]
    InvalidInstructionDefinition(String),
    #[error("invalid instruction definition: {0}")]
    InvalidMethodDefinition(String),
    #[error("type descriptor is empty")]
    EmptyTypeDescriptor,
    #[error(transparent)]
    Descriptor {
        #[from]
        source: DescriptorError
    },
    #[error(transparent)]
    Signature {
        #[from]
        source: SignatureError
    },
    #[error(transparent)]
    Name {
        #[from]
        source: NameError
    },
    #[error(transparent)]
    Number {
        #[from]
        source: ParseNumberError
    },
}

#[derive(Error, Debug)]
pub enum ParseNumberError {
    #[error("not int or float: i - {i_error}, f - {f_error}")]
    NotIntOrFloat {
        i_error: ParseIntError,
        f_error: ParseFloatError
    },
    #[error("not long or double: l - {i_error}, d - {f_error}")]
    NotLongOrDouble {
        i_error: ParseIntError,
        f_error: ParseFloatError
    },
    #[error("invalid number value: {source}")]
    Int {
        #[from]
        source: ParseIntError
    },
    #[error("invalid number value: {source}")]
    Float {
        #[from]
        source: ParseFloatError
    },
}


#[derive(Error, Debug)]
pub enum CreateClassError {
    #[error(transparent)]
    Class {
        #[from]
        source: ClassError
    },
    #[error(transparent)]
    Method {
        #[from]
        source: MethodError
    },
    #[error(transparent)]
    Code {
        #[from]
        source: CodeError
    },
    #[error(transparent)]
    Signature {
        #[from]
        source: SignatureError
    },
}