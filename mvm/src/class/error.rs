use thiserror::Error;
use crate::types::error::ValueError;
use crate::class::descriptor::{ReturnDescriptor, TypeDescriptor, ParamsDescriptor};
use crate::class::name::MethodName;


#[derive(Error, Debug)]
pub enum ArrayError {
    #[error("array type mismatch")]
    TypeMismatch,
    #[error("index is out of bounds")]
    IndexOutOfBounds
}


#[derive(Error, Debug)]
pub enum NameError {
    #[error("class name is invalid")]
    InvalidClassName,
    #[error("method name \"{name:?}\" is invalid")]
    InvalidMethodName{
        name: String
    },
    #[error("field name is invalid")]
    InvalidFieldName,
}


#[derive(Error, Debug)]
pub enum CodeError {
    #[error("instruction index is out of bounds: the length is {max} but the index is {index}]")]
    IndexOutOfBounds {
        max: usize,
        index: usize
    },
    #[error("there are no instructions in the code")]
    NoInstructions,
}


#[derive(Error, Debug)]
pub enum DescriptorError {
    #[error("number of dimensions must not be 0")]
    ZeroArrayDimension,
    #[error("max number of dimensions is {max}, {given} given")]
    TooManyDimensions {
        max: usize,
        given: usize
    },
}


#[derive(Error, Debug)]
pub enum ConstantPoolError {
    #[error("index is out of bounds")]
    IndexOutOfBounds,
    #[error(transparent)]
    Value {
        #[from]
        source: ValueError
    },
}


#[derive(Error, Debug)]
pub enum ClassError {
    #[error("no such method was found in class")]
    NoSuchMethod,
    #[error("no such field was found in class")]
    NoSuchField,
    #[error("multiple definitions of the same method: {return_desc} {name}{params_desc}")]
    DuplicateMethod {
        return_desc: ReturnDescriptor,
        name: MethodName,
        params_desc: ParamsDescriptor
    },
    #[error("multiple definitions of the same field")]
    DuplicateField,
}

#[derive(Error, Debug)]
pub enum MethodError {
    #[error("there are too few locals entries available for the method")]
    TooFewLocalsEntries,
}

#[derive(Error, Debug)]
pub enum FieldError {}