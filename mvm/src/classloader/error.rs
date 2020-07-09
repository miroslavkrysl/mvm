use thiserror::Error;
use std::num::{ParseIntError, ParseFloatError};
use crate::class::error::{NameError, ArrayError};


#[derive(Error, Debug)]
pub enum ClassParserError {
    #[error(transparent)]
    Name {
        source: NameError
    },
    #[error(transparent)]
    Int(ParseIntError),
    #[error(transparent)]
    Float(ParseFloatError),
    #[error("unknown instruction: {0:?}")]
    UnknownInstruction(String),
    #[error(transparent)]
    Array(ArrayError)
}
