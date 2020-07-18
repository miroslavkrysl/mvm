use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;
use crate::vm::types::int::Int;
use crate::vm::types::float::Float;
use crate::vm::types::double::Double;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;
use crate::vm::exec::vm::VmEvent;


impl Instruction {
    pub(super) fn istore(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals().store(index as usize, value)?;
        thread.runtime().emit_event(VmEvent::LocalsChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lstore(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Long>()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals().store(index as usize, value)?;
        thread.runtime().emit_event(VmEvent::LocalsChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fstore(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Float>()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals().store(index as usize, value)?;
        thread.runtime().emit_event(VmEvent::LocalsChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dstore(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Double>()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals().store(index as usize, value)?;
        thread.runtime().emit_event(VmEvent::LocalsChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn astore(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Reference>()?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.locals().store(index as usize, value)?;
        thread.runtime().emit_event(VmEvent::LocalsChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn istore_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.istore(thread, 0)
    }

    pub(super) fn istore_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.istore(thread, 1)
    }

    pub(super) fn istore_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.istore(thread, 2)
    }

    pub(super) fn istore_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.istore(thread, 3)
    }

    pub(super) fn lstore_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lstore(thread, 0)
    }

    pub(super) fn lstore_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lstore(thread, 1)
    }

    pub(super) fn lstore_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lstore(thread, 2)
    }

    pub(super) fn lstore_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lstore(thread, 3)
    }

    pub(super) fn fstore_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fstore(thread, 0)
    }

    pub(super) fn fstore_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fstore(thread, 1)
    }

    pub(super) fn fstore_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fstore(thread, 2)
    }

    pub(super) fn fstore_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fstore(thread, 3)
    }

    pub(super) fn dstore_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dstore(thread, 0)
    }

    pub(super) fn dstore_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dstore(thread, 1)
    }

    pub(super) fn dstore_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dstore(thread, 2)
    }

    pub(super) fn dstore_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dstore(thread, 3)
    }

    pub(super) fn astore_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.astore(thread, 0)
    }

    pub(super) fn astore_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.astore(thread, 1)
    }

    pub(super) fn astore_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.astore(thread, 2)
    }

    pub(super) fn astore_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.astore(thread, 3)
    }
}