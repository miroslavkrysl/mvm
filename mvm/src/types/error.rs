use thiserror::Error;

#[derive(Error, Debug)]
#[error("division by zero")]
pub struct DivisionByZero;

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("unexpected value type")]
    UnexpectedType,
    #[error("unexpected value category")]
    UnexpectedCategory,
}