use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArrayError {
    #[error("array type mismatch")]
    ItemTypeMismatch
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
pub enum FlagsError {
    #[error("invalid flags combination")]
    InvalidCombination,
}