use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;
use crate::vm::types::double::Double;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;


impl Instruction {
    pub(super) fn iload(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Int>(index as usize)?;
        frame.stack().push(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lload(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Long>(index as usize)?;
        frame.stack().push(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fload(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Float>(index as usize)?;
        frame.stack().push(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dload(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Double>(index as usize)?;
        frame.stack().push(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn aload(&self, thread: &Thread, index: u8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Reference>(index as usize)?;
        frame.stack().push(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn iload_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.iload(thread, 0)
    }

    pub(super) fn iload_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.iload(thread, 1)
    }

    pub(super) fn iload_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.iload(thread, 2)
    }

    pub(super) fn iload_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.iload(thread, 3)
    }

    pub(super) fn lload_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lload(thread, 0)
    }

    pub(super) fn lload_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lload(thread, 1)
    }

    pub(super) fn lload_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lload(thread, 2)
    }

    pub(super) fn lload_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.lload(thread, 3)
    }

    pub(super) fn fload_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fload(thread, 0)
    }

    pub(super) fn fload_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fload(thread, 1)
    }

    pub(super) fn fload_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fload(thread, 2)
    }

    pub(super) fn fload_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.fload(thread, 3)
    }

    pub(super) fn dload_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dload(thread, 0)
    }

    pub(super) fn dload_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dload(thread, 1)
    }

    pub(super) fn dload_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dload(thread, 2)
    }

    pub(super) fn dload_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.dload(thread, 3)
    }

    pub(super) fn aload_0(&self, thread: &Thread) -> Result<(), ExecError> {
        self.aload(thread, 0)
    }

    pub(super) fn aload_1(&self, thread: &Thread) -> Result<(), ExecError> {
        self.aload(thread, 1)
    }

    pub(super) fn aload_2(&self, thread: &Thread) -> Result<(), ExecError> {
        self.aload(thread, 2)
    }

    pub(super) fn aload_3(&self, thread: &Thread) -> Result<(), ExecError> {
        self.aload(thread, 3)
    }
}