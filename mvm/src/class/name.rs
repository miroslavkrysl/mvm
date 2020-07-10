use std::fmt;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

use crate::class::error::NameError;
use std::convert::TryFrom;

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
    pub fn new<S>(name: S) -> Result<Self, NameError>
                  where S: Into<String> {
        let name = name.into();

        if !CLASS_NAME_REGEX.is_match(&name) {
            return Err(NameError::InvalidClassName)
        }

        Ok(ClassName { name })
    }
}

impl AsRef<str> for ClassName {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
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

    pub fn new<S>(name: S) -> Result<Self, NameError> where S: Into<String> {
        let name = name.into();
        if name != Self::INSTANCE_INIT_STRING && name != Self::CLASS_INIT_STRING && !METHOD_NAME_REGEX.is_match(&name) {
            return Err(NameError::InvalidMethodName { name });
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
    pub fn new<S>(name: S) -> Result<Self, NameError> where S: Into<String> {
        let name = name.into();

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
