use std::path::PathBuf;
use std::fmt;
use std::iter::FromIterator;
use crate::structs::error::NameError;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name {
    name: String
}

impl Name {
    pub const INVALID_CHARS: [char; 4] = ['.', ';', '[', '/'];

    pub fn new(name: String) -> Result<Self, NameError> {
        if name.is_empty() {
            return Err(NameError::Empty);
        }

        if name.chars().all(|c| !Self::INVALID_CHARS.contains(&c)) {
            return Err(NameError::InvalidChars);
        }

        Ok(Name { name })
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitKind {
    None,
    Init,
    ClassInit,
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodName {
    name: String,
    kind: InitKind
}

impl MethodName {
    pub const INVALID_CHARS: [char; 6] = ['.', ';', '[', '/', '<', '>'];
    pub const INIT_STRING: &'static str = "<init>";
    pub const CLASS_INIT_STRING: &'static str= "<clinit>";

    pub fn new(name: String) -> Result<Self, NameError> {
        if name.is_empty() {
            return Err(NameError::Empty);
        }

        let (kind, start) = if name.starts_with(Self::INIT_STRING) {
            (InitKind::Init, Self::INIT_STRING.len())
        } else if name.starts_with(Self::CLASS_INIT_STRING) {
            (InitKind::ClassInit, Self::CLASS_INIT_STRING.len())
        } else {
            (InitKind::None, 0)
        };

        if name.chars().skip(start).all(|c| !Self::INVALID_CHARS.contains(&c)) {
            return Err(NameError::InvalidChars);
        }

        Ok(MethodName { name, kind })
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FullName {
    parts: Vec<Name>
}

impl FullName {
    pub fn new() -> Self {
        FullName {
            parts: Vec::new()
        }
    }

    pub fn to_path_buff(&self) -> PathBuf {
        self.parts.iter().collect()
    }
}

impl Extend<Name> for FullName {
    fn extend<T: IntoIterator<Item=Name>>(&mut self, iter: T) {
        self.parts.extend(iter)
    }
}

impl FromIterator<Name> for FullName {
    fn from_iter<T: IntoIterator<Item=Name>>(iter: T) -> Self {
        let mut name = FullName::new();
        name.extend(iter);
        name
    }
}

impl fmt::Display for FullName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parts.join("."))
    }
}