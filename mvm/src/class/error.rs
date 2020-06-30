use thiserror::Error;
use crate::types::ValueError;


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
    #[error("method name is invalid")]
    InvalidMethodName,
    #[error("field name is invalid")]
    InvalidFieldName,
}


#[derive(Error, Debug)]
pub enum CodeError {
    #[error("index is out of bounds")]
    IndexOutOfBounds,
}


#[derive(Error, Debug)]
pub enum DescriptorError {
    #[error("dimension must be non-zero")]
    ZeroArrayDimension,
}


#[derive(Error, Debug)]
pub enum FlagsError {
    #[error("invalid flags combination")]
    InvalidCombination,
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
    #[error("multiple definitions of the same method")]
    DuplicateMethod,
    #[error("multiple definitions of the same field")]
    DuplicateField,
}

#[derive(Error, Debug)]
pub enum MethodError {
    #[error("init method has invalid properties")]
    InvalidInitProperties,
    #[error("class init method has invalid properties")]
    InvalidClassInitProperties,
}