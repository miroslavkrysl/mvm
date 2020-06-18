
#[derive(Debug, Copy, Clone)]
pub struct Boolean {
    value: u8
}

impl Boolean {
    pub fn new(value: u8) -> Self {
        Boolean {
            value
        }
    }
}

impl From<Boolean> for u8 {
    fn from(boolean: Boolean) -> Self {
        boolean.value
    }
}
