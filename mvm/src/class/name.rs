use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use crate::class::error::NameError;

lazy_static! {
    static ref CLASS_NAME_REGEX: Regex = Regex::new(r"([^.;\[/]+\.)*[^.;\[/]+").unwrap();
    static ref METHOD_NAME_REGEX: Regex = Regex::new(r"[^.;\[/<>]+").unwrap();
    static ref FIELD_NAME_REGEX: Regex = Regex::new(r"[^.;\[/]+").unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ClassName {
    name: String
}

impl ClassName {
    pub fn new(name: String) -> Result<Self, NameError> {
        if !CLASS_NAME_REGEX.is_match(&name) {
            return Err(NameError::InvalidClassName);
        }

        Ok(ClassName { name })
    }
}

impl fmt::Display for ClassName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.escape_debug())
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MethodName {
    name: String,
}

impl MethodName {
    pub const INSTANCE_INIT_STRING: &'static str = "<init>";
    pub const CLASS_INIT_STRING: &'static str = "<clinit>";

    pub fn new(name: String) -> Result<Self, NameError> {
        if name != Self::INSTANCE_INIT_STRING || name != Self::CLASS_INIT_STRING || !METHOD_NAME_REGEX.is_match(&name) {
            return Err(NameError::InvalidMethodName);
        }

        Ok(MethodName { name })
    }

    pub fn is_init(&self) -> bool {
        self.name == Self::INSTANCE_INIT_STRING
    }

    pub fn is_class_init(&self) -> bool {
        self.name == Self::CLASS_INIT_STRING
    }
}

impl fmt::Display for MethodName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.escape_debug())
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FieldName {
    name: String
}

impl FieldName {
    pub fn new(name: String) -> Result<Self, NameError> {
        if !FIELD_NAME_REGEX.is_match(&name) {
            return Err(NameError::InvalidFieldName);
        }

        Ok(FieldName { name })
    }
}

impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.escape_debug())
    }
}
