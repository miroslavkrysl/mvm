#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ValueCategory {
    Single,
    Double,
}

impl ValueCategory {
    pub fn size(&self) -> usize {
        match self {
            ValueCategory::Single => 1,
            ValueCategory::Double => 2,
        }
    }
}

pub trait Categorize {
    fn category(&self) -> ValueCategory;
}