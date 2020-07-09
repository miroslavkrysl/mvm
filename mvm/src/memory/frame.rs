use std::sync::Arc;

use crate::class::Method;
use crate::memory::{Locals, FrameError};
use crate::memory::OperandStack;
use crate::types::JvmValue;
use regex::internal::Input;

pub struct Frame {
    method: Arc<Method>,
    stack: OperandStack,
    locals: Locals,
    pc: usize,
}

impl Frame {
    // pub fn new<I: IntoIterator<Item=JvmValue>>(method: Arc<Method>, args: I) -> Self {
    //     let locals = Locals::new(method.code().max_locals());
    //
    //
    //     Ok(Frame {
    //         method,
    //         stack: OperandStack::new(method.code().max_stack()),
    //         locals,
    //         pc: 0,
    //     })
    // }

    pub fn stack(&self) -> &OperandStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut OperandStack {
        &mut self.stack
    }

    pub fn locals(&self) -> &Locals {
        &self.locals
    }

    pub fn locals_mut(&mut self) -> &mut Locals {
        &mut self.locals
    }

    pub fn method(&self) -> &Arc<Method> {
        &self.method
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1
    }
}