use std::path::PathBuf;
use std::fmt;
use std::iter::FromIterator;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Name {
    name: String
}

impl Name {
    pub fn new(name: String) -> Self {
        // TODO: check regex
        Name {
            name
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
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