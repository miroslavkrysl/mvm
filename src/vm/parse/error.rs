//! Parsing errors.

use std::fmt;
use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};

use thiserror::Error;

use crate::vm::class::error::{ClassError, CodeError, DescriptorError, MethodError, NameError, SignatureError};


/// An error that can occur while parsing a class file.
#[derive(Error, Debug)]
pub struct ParseClassError {
    kind: ParseClassErrorKind,
    line_pos: usize,
}


impl ParseClassError {
    /// Creates a new `ParseClassError` from given kind and line position.
    pub fn new(kind: ParseClassErrorKind, line_pos: usize) -> Self {
        ParseClassError {
            kind,
            line_pos,
        }
    }
}


impl Display for ParseClassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "on line {}: {}", self.line_pos, self.kind)
    }
}


/// A kind of error that can occur while parsing a class file.
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


/// An error that can occur while parsing a number.
#[derive(Error, Debug)]
pub enum ParseNumberError {
    #[error("not int or float: i - {i_error}, f - {f_error}")]
    NotIntOrFloat {
        i_error: ParseIntError,
        f_error: ParseFloatError,
    },
    #[error("not long or double: l - {i_error}, d - {f_error}")]
    NotLongOrDouble {
        i_error: ParseIntError,
        f_error: ParseFloatError,
    },
    #[error("invalid integer value: {source}")]
    Int {
        #[from]
        source: ParseIntError
    },
    #[error("invalid float value: {source}")]
    Float {
        #[from]
        source: ParseFloatError
    },
}


/// An error that can occur while creating a representation of class
/// from class file representation.
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