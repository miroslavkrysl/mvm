use thiserror::Error;

#[derive(Error, Debug)]
#[error("division by zero")]
pub struct DivisionByZero;
