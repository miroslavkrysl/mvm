use std::sync::{Arc, RwLock};

use crate::vm::class::class::Class;
use crate::vm::class::method::Method;
use crate::vm::memory::error::FrameError;
use crate::vm::memory::locals::Locals;
use crate::vm::memory::operand_stack::OperandStack;
use crate::vm::types::reference::Reference;
use crate::vm::types::value::ValueType;


/// A method frame.
/// Every frame has its own locasl, operand stack
/// and pc register.
pub struct Frame {
    class: Arc<Class>,
    method: Arc<Method>,
    stack: OperandStack,
    locals: Locals,
    pc: RwLock<isize>,
}


impl Frame {
    pub const MAX_STACK: usize = 255;

    /// Create a new frame for method.
    pub fn new(class: Arc<Class>, method: Arc<Method>) -> Self {
        let locals = Locals::new(method.code().locals_size());

        Frame {
            class,
            method,
            stack: OperandStack::new(Self::MAX_STACK),
            locals,
            pc: RwLock::new(0),
        }
    }

    /// Create a new frame for method, pop arguments from stack
    /// and load them to locals of the new frame.
    pub fn new_from_call(class: Arc<Class>, method: Arc<Method>, stack: &OperandStack) -> Result<Self, FrameError> {
        let locals = Locals::new(method.code().locals_size());

        // pop arguments from stack and load them to locals
        let mut index = method.signature().params_desc().size();

        if !method.is_static() {
            index += ValueType::AnyReference.category().size();
        }

        for type_desc in method.signature().params_desc().type_descs().iter().rev() {
            let value = &stack.peek_value(0)?;
            if !type_desc.is_assignable_with(value) {
                return Err(FrameError::IncompatibleArgumentType {
                    expected: type_desc.clone(),
                    got: value.value_type(),
                });
            }

            let value = stack.pop_value().unwrap();
            let size = value.value_type().category().size();
            locals.store_value(index - size, value).unwrap();
            index -= size;
        }
        if !method.is_static() {
            let this = stack.pop::<Reference>()?;
            locals.store(0, this).unwrap();
        }

        Ok(Frame {
            class,
            method,
            stack: OperandStack::new(Self::MAX_STACK),
            locals,
            pc: RwLock::new(0),
        })
    }

    /// Returns the stack of this Frame.
    pub fn stack(&self) -> &OperandStack {
        &self.stack
    }

    /// Returns the loacals of this Frame.
    pub fn locals(&self) -> &Locals {
        &self.locals
    }

    /// Returns the method of this Frame.
    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    /// Returns the class of this Frame.
    pub fn class(&self) -> &Arc<Class> {
        &self.class
    }

    /// Returns the pc of this Frame.
    pub fn pc(&self) -> isize {
        self.pc.read().unwrap().clone()
    }

    /// Increment the pc of this Frame by one.
    pub fn inc_pc(&self) {
        *self.pc.write().unwrap() += 1
    }

    /// Sets the value of the pc of this Frame as current pc + offset.
    pub fn offset_pc(&self, offset: i16) {
        *self.pc.write().unwrap() += offset as isize;
    }
}