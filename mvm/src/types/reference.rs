
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Reference {
    value: usize
}

impl Reference {
    const NULL_VALUE: usize = 0;

    pub fn new(value: usize) -> Self {
        Reference {
            value
        }
    }

    pub fn null() -> Self {
        Reference {
            value: 0
        }
    }

    pub fn is_null(&self) -> bool {
        self.value == Self::NULL_VALUE
    }

    pub fn eq(&self, other: Reference) -> bool {
        self.value == other.value
    }
}
