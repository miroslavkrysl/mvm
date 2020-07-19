use std::sync::{Arc, RwLock};
use crate::vm::class::method::Method;
use crate::vm::memory::operand_stack::OperandStack;
use crate::vm::memory::locals::Locals;
use crate::vm::memory::error::FrameError;
use crate::vm::class::class::Class;
use crate::vm::types::reference::Reference;
use crate::vm::exec::error::ExecError;


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
        let mut locals = Locals::new(method.code().locals_size());

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
        let mut index = if method.is_static() {0} else {1};
        for type_desc in method.signature().params_desc().type_descs() {
            if !type_desc.is_assignable_with( &stack.peek_value(0)?) {
                return Err(FrameError::IncompatibleArgumentType {
                    expected: type_desc.clone()
                });
            }

            let value = stack.pop_value().unwrap();
            let size = value.value_type().category().size();
            locals.store_value(index, value).unwrap();
            index += size;
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

    pub fn stack(&self) -> &OperandStack {
        &self.stack
    }

    pub fn locals(&self) -> &Locals {
        &self.locals
    }

    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    pub fn pc(&self) -> isize {
        self.pc.read().unwrap().clone()
    }

    pub fn inc_pc(&self) {
        *self.pc.write().unwrap() += 1
    }

    pub fn offset_pc(&self, offset: i16) {
        *self.pc.write().unwrap() += offset as isize;
    }
}