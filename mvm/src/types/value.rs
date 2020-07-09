use crate::types::{JvmValue, CompValue};

pub trait Value: Into<JvmValue> + Into<CompValue> {

}

impl TryFrom<JvmValue> for Int {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Int(int) => Ok(int),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Long {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Long(long) => Ok(long),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Float {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Float(float) => Ok(float),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Double {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Double(double) => Ok(double),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Byte {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Byte(byte) => Ok(byte),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Short {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Short(short) => Ok(short),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Boolean {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Boolean(boolean) => Ok(boolean),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Char {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Char(char) => Ok(char),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl TryFrom<JvmValue> for Reference {
    type Error = ValueError;

    fn try_from(value: JvmValue) -> Result<Self, Self::Error> {
        match value {
            JvmValue::Reference(reference) => Ok(reference),
            _ => Err(ValueError::UnexpectedType),
        }
    }
}


impl From<Int> for JvmValue {
    fn from(int: Int) -> Self {
        JvmValue::Int(int)
    }
}


impl From<Long> for JvmValue {
    fn from(long: Long) -> Self {
        JvmValue::Long(long)
    }
}


impl From<Float> for JvmValue {
    fn from(float: Float) -> Self {
        JvmValue::Float(float)
    }
}


impl From<Double> for JvmValue {
    fn from(double: Double) -> Self {
        JvmValue::Double(double)
    }
}


impl From<Reference> for JvmValue {
    fn from(reference: Reference) -> Self {
        JvmValue::Reference(reference)
    }
}


impl From<Byte> for JvmValue {
    fn from(byte: Byte) -> Self {
        JvmValue::Byte(byte)
    }
}


impl From<Short> for JvmValue {
    fn from(short: Short) -> Self {
        JvmValue::Short(short)
    }
}


impl From<Boolean> for JvmValue {
    fn from(boolean: Boolean) -> Self {
        JvmValue::Boolean(boolean)
    }
}


impl From<Char> for JvmValue {
    fn from(char: Char) -> Self {
        JvmValue::Char(char)
    }
}


impl fmt::Display for JvmValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JvmValue::Int(value) => write!(f, "{}", value),
            JvmValue::Long(value) => write!(f, "{}", value),
            JvmValue::Float(value) => write!(f, "{}", value),
            JvmValue::Double(value) => write!(f, "{}", value),
            JvmValue::Reference(value) => write!(f, "{}", value),
            JvmValue::Byte(value) => write!(f, "{}", value),
            JvmValue::Short(value) => write!(f, "{}", value),
            JvmValue::Boolean(value) => write!(f, "{}", value),
            JvmValue::Char(value) => write!(f, "{}", value),
        }
    }
}