use thiserror::Error;

use crate::vm::class::descriptor::{ParamsDesc, ReturnDesc};
use crate::vm::class::name::{ClassName, MethodName};
use crate::vm::class::signature::{FieldSig, MethodSig};
use crate::vm::types::int::Int;
use crate::vm::types::value::{Value, ValueType};


/// An error that can occur while creating a class, field or method name.
#[derive(Error, Debug)]
pub enum NameError {
    #[error("class name {0:?} is invalid")]
    InvalidClassName(String),
    #[error("method name {0:?} is invalid")]
    InvalidMethodName(String),
    #[error("field name {0:?} is invalid")]
    InvalidFieldName(String),
}


/// An error that can occur while creating a field or method signature.
#[derive(Error, Debug)]
pub enum SignatureError {
    #[error("invalid instance initialization method signature")]
    InvalidInitSignature,
    #[error("invalid class initialization method signature")]
    InvalidClinitSignature,
}


/// An error that can occur while creating a descriptor.
#[derive(Error, Debug)]
pub enum DescriptorError {
    #[error("number of dimensions must not be 0")]
    ZeroArrayDimensions,
    #[error("max number of dimensions is {max} but {dim} given")]
    TooManyDimensions {
        max: usize,
        dim: usize,
    },
}


#[derive(Error, Debug)]
pub enum CodeError {
    #[error("instruction index is out of bounds: the length is {max} but the index is {index}]")]
    IndexOutOfBounds {
        max: usize,
        index: isize,
    },
    #[error("the size of locals {size} is too big, max is {max}")]
    TooBigLocalsSize {
        size: usize,
        max: usize,
    },
    #[error("there are no instructions in the code")]
    NoInstructions,
}


#[derive(Error, Debug)]
pub enum MethodError {
    #[error("there are {locals_size} locals entries but the method params needs {params_size}")]
    TooFewLocalsEntries {
        locals_size: usize,
        params_size: usize,
    },
    #[error("instance initialization method can not be static")]
    InitIsStatic,
    #[error("class initialization method must be static")]
    ClinitIsNonStatic,
}


#[derive(Error, Debug)]
pub enum ClassError {
    #[error("no such field was found in class")]
    NoSuchField(FieldSig),
    #[error("no such method was found in class")]
    NoSuchMethod(MethodSig),
    #[error("multiple definitions of the same field \"{0}\"")]
    DuplicateField(FieldSig),
    #[error("multiple definitions of the same method \"{0}\"")]
    DuplicateMethod(MethodSig),
    #[error("can not assign value {1} of type {} to field \"{0}\"", .1.value_type())]
    FieldValueTypeMismatch(FieldSig, Value),
    #[error("class \"{0}\" of instance is not a subclass of of \"{1}\"")]
    NotInstanceOf(ClassName, ClassName),
}

