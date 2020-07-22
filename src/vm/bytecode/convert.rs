use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;
use crate::vm::types::double::Double;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;


impl Instruction {
    pub(super) fn i2l(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        let result = value.to_long();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn i2f(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        let result = value.to_float();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn i2d(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        let result = value.to_double();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn l2i(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Long>()?;
        let result = value.to_int();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn l2f(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Long>()?;
        let result = value.to_float();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn l2d(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Long>()?;
        let result = value.to_double();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn f2i(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Float>()?;
        let result = value.to_int();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn f2l(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Float>()?;
        let result = value.to_long();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn f2d(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Float>()?;
        let result = value.to_double();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn d2i(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Double>()?;
        let result = value.to_int();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn d2l(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Double>()?;
        let result = value.to_long();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn d2f(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Double>()?;
        let result = value.to_float();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }
}