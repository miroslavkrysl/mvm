//! Class, field, and method names.

use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

use crate::vm::class::error::NameError;


/// A class name.
/// The name may consist of multiple identifiers separated by dots.
/// The identifiers must have at least one character and must not
/// contain any of the characters {`. ; [ ]`}.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ClassName {
    name: String
}


impl ClassName {
    /// Create a new class name.
    ///
    /// # Errors
    ///
    /// Returns `NameError::InvalidClassName` if the name is invalid.
    pub fn new<S>(name: S) -> Result<Self, NameError>
                  where S: Into<String> {
        let name = name.into();

        if !Self::regex().is_match(&name) {
            return Err(NameError::InvalidClassName(name));
        }

        Ok(ClassName { name })
    }

    pub fn regex() -> &'static Regex {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^([^\.;\[/]+\.)*[^\.;\[/]+$").unwrap();
        }

        &REGEX
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


/// A method name.
/// The name must have at least one character and must not
/// contain any of the characters {`. ; [ ] < >`} except for
/// special method names `<init>` and `<clinit>`.
impl MethodName {
    pub const INIT_STRING: &'static str = "<init>";
    pub const CLINIT_STRING: &'static str = "<clinit>";

    /// Create a new method name.
    ///
    /// # Errors
    ///
    /// Returns `NameError::InvalidMethodName` if the name is invalid.
    pub fn new<S>(name: S) -> Result<Self, NameError> where S: Into<String> {
        let name = name.into();

        if &name != Self::INIT_STRING
            && &name != Self::CLINIT_STRING
            && !Self::regex().is_match(&name) {
            return Err(NameError::InvalidMethodName(name));
        }

        Ok(MethodName { name })
    }

    pub fn regex() -> &'static Regex {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^[^\.;\[/<>]+$").unwrap();
        }

        &REGEX
    }

    /// Returns true if the name is of instance initialization method, false otherwise.
    pub fn is_init(&self) -> bool {
        &self.name == Self::INIT_STRING
    }

    /// Returns true if the name is of class initialization method, false otherwise.
    pub fn is_clinit(&self) -> bool {
        &self.name == Self::CLINIT_STRING
    }
}


impl AsRef<str> for MethodName {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
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


/// A field name.
/// The name must have at least one character and must not
/// contain any of the characters {`. ; [ ]`}.
impl FieldName {
    /// Create a new field name.
    ///
    /// # Errors
    ///
    /// Returns `NameError::InvalidFieldName` if the name is invalid.
    pub fn new<S>(name: S) -> Result<Self, NameError> where S: Into<String> {
        let name = name.into();

        if !Self::regex().is_match(&name) {
            return Err(NameError::InvalidFieldName(name));
        }

        Ok(FieldName { name })
    }

    /// Get the field name regex.
    #[inline]
    pub fn regex() -> &'static Regex {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^[^\.;\[/]+$").unwrap();
        }
        &REGEX
    }
}


impl AsRef<str> for FieldName {
    fn as_ref(&self) -> &str {
        self.name.as_ref()
    }
}


impl fmt::Display for FieldName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name.escape_debug())
    }
}
