use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::vm::VmEvent;


impl Instruction {
    pub(super) fn pop(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().pop_discard1()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn pop2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().pop_discard2()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup1()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup_x1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup1_skip1()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup_x2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup1_skip2()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup2()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup2_x1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup2_skip1()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn dup2_x2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().dup2_skip2()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }

    pub(super) fn swap(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.stack().swap()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals();
        Ok(())
    }
}