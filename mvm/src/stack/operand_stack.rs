// use crate::types::{Int, Long, Float, Double, Reference};
// use crate::stack::error::OperandStackError;
//
//
// #[derive(Debug, Copy, Clone)]
// pub enum OperandStackValue {
//     Int(Int),
//     Long(Long),
//     Float(Float),
//     Double(Double),
//     Reference(Reference)
// }
//
//
// #[derive(Debug)]
// pub struct OperandStack {
//     data: Vec<OperandStackValue>,
//     size: usize,
//     capacity: u16,
// }
//
// impl OperandStack {
//     pub fn new(capacity: u16) -> Self {
//         OperandStack {
//             data: Vec::with_capacity(capacity as usize),
//             size: 0,
//             capacity,
//         }
//     }
//
//     pub fn push_int(&mut self, value: Int) -> Result<(), OperandStackError> {
//         self.check_overflow(1)?;
//
//         self.data.push(OperandStackValue::Int(value));
//         self.size += 1;
//
//         Ok(())
//     }
//
//     pub fn push_long(&mut self, value: Long) -> Result<(), OperandStackError> {
//         self.check_overflow(2)?;
//
//         self.data.push(OperandStackValue::Long(value));
//         self.size += 2;
//
//         Ok(())
//     }
//
//     pub fn push_float(&mut self, value: Float) -> Result<(), OperandStackError> {
//         self.check_overflow(1)?;
//
//         self.data.push(OperandStackValue::Float(value));
//         self.size += 1;
//
//         Ok(())
//     }
//
//     pub fn push_double(&mut self, value: Double) -> Result<(), OperandStackError> {
//         self.check_overflow(2)?;
//
//         self.data.push(OperandStackValue::Double(value));
//         self.size += 2;
//
//         Ok(())
//     }
//
//     pub fn push_reference(&mut self, value: Reference) -> Result<(), OperandStackError> {
//         self.check_overflow(1)?;
//
//         self.data.push(OperandStackValue::Reference(value));
//         self.size += 1;
//
//         Ok(())
//     }
//
//     pub fn pop_int(&mut self) -> Result<Int, OperandStackError> {
//         self.check_underflow()?;
//
//         if let OperandStackValue::Int(value) = self.data.last().unwrap() {
//             self.data.pop();
//             self.size -= 1;
//             Ok(*value)
//         } else {
//             Err(OperandStackError::NonMatchingTypes)
//         }
//     }
//
//     pub fn pop_long(&mut self) -> Result<Long, OperandStackError> {
//         self.check_underflow()?;
//
//         if let OperandStackValue::Long(value) = self.data.last().unwrap() {
//             self.data.pop();
//             self.size -= 2;
//             Ok(*value)
//         } else {
//             Err(OperandStackError::NonMatchingTypes)
//         }
//     }
//
//     pub fn pop_float(&mut self) -> Result<Float, OperandStackError> {
//         self.check_underflow()?;
//
//         if let OperandStackValue::Float(value) = self.data.last().unwrap() {
//             self.data.pop();
//             self.size -= 1;
//             Ok(*value)
//         } else {
//             Err(OperandStackError::NonMatchingTypes)
//         }
//     }
//
//     pub fn pop_double(&mut self) -> Result<Double, OperandStackError> {
//         self.check_underflow()?;
//
//         if let OperandStackValue::Double(value) = self.data.last().unwrap() {
//             self.data.pop();
//             self.size -= 2;
//             Ok(*value)
//         } else {
//             Err(OperandStackError::NonMatchingTypes)
//         }
//     }
//
//     pub fn pop_reference(&mut self) -> Result<Reference, OperandStackError> {
//         self.check_underflow()?;
//
//         if let OperandStackValue::Reference(value) = self.data.last().unwrap() {
//             self.data.pop();
//             self.size -= 1;
//             Ok(*value)
//         } else {
//             Err(OperandStackError::NonMatchingTypes)
//         }
//     }
//
//     fn check_overflow(&self, reserve: usize) -> Result<(), OperandStackError> {
//         if self.size + reserve > self.capacity as usize {
//             return Err(OperandStackError::StackOverflow);
//         } else {
//             Ok(())
//         }
//     }
//
//     fn check_underflow(&self) -> Result<(), OperandStackError> {
//         if self.size == 0 {
//             return Err(OperandStackError::StackUnderflow);
//         } else {
//             Ok(())
//         }
//     }
// }
