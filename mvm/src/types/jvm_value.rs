use crate::types::{Byte, Short, Int, Long, Float, Double, Boolean, Char, Reference};

#[derive(Debug, Copy, Clone)]
pub enum JvmValue {
    Byte(Byte),
    Short(Short),
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Boolean(Boolean),
    Char(Char),
    Reference(Reference),
}
