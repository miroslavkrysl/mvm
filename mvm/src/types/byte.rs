#[derive(Debug, Copy, Clone)]
pub struct Byte {
    value: i8
}

impl Byte {
    pub fn new(value: i8) -> Self {
        Byte {
            value
        }
    }
}

impl From<Byte> for i8 {
    fn from(byte: Byte) -> Self {
        byte.value
    }
}
