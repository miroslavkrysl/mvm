
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperandSize {
    Single,
    Double
}

impl From<OperandSize> for usize {
    fn from(value: OperandSize) -> Self {
        match value {
            OperandSize::Single => 1,
            OperandSize::Double => 2,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Int(Int),
    Long(Long),
    Float(Float),
    Double(Double),
    Reference(Reference),
}


impl Operand {
    pub fn size(&self) -> OperandSize {
        match self {
            Operand::Int(_) => OperandSize::Single,
            Operand::Long(_) => OperandSize::Double,
            Operand::Float(_) => OperandSize::Single,
            Operand::Double(_) => OperandSize::Double,
            Operand::Reference(_) => OperandSize::Single,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Reference {

}


#[derive(Debug, Copy, Clone)]
pub struct Int(i32);

impl Int {
    pub fn new(value: i32) -> Self {
        Int(value)
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Long(i64);


#[derive(Debug, Copy, Clone)]
pub struct Float(f32);


#[derive(Debug, Copy, Clone)]
pub struct Double(f64);

impl Double {
    pub fn new(value: f64) -> Self {
        Double(value)
    }
}


// #[derive(Debug, Copy, Clone)]
// pub enum Reference {
//     Instance,
//     Array{
//         kind: ArrayKind
//     },
//     Null
// }


#[derive(Debug, Copy, Clone)]
pub struct Byte(i8);


#[derive(Debug, Copy, Clone)]
pub struct Short(i16);


#[derive(Debug, Copy, Clone)]
pub struct Boolean(u8);


#[derive(Debug, Copy, Clone)]
pub struct Char(u16);
