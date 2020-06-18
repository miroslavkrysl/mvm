#[derive(Debug, Copy, Clone)]
pub struct Short {
    value: i16
}

impl Short {
    pub fn new(value: i16) -> Self {
        Short {
            value
        }
    }
}

impl From<Short> for i16 {
    fn from(short: Short) -> Self {
        short.value
    }
}
