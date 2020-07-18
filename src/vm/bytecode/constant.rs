use crate::vm::bytecode::instruction::{Instruction, Ldc2Arg, LdcArg};
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::vm::VmEvent;
use crate::vm::types::double::Double;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;
use crate::vm::types::value::Value;


impl Instruction {
    pub(super) fn aconst_null(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Reference::null();
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_m1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(-1);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_0(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(1);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(2);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_3(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(3);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_4(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(4);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn iconst_5(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(5);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn lconst_0(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Long::new(0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn lconst_1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Long::new(1);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn fconst_0(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Float::new(0.0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn fconst_1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Float::new(1.0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn fconst_2(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Float::new(2.0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn dconst_0(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Double::new(0.0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn dconst_1(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Double::new(1.0);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn bipush(&self, thread: &Thread, byte: i8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(byte as i32);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }


    pub(super) fn sipush(&self, thread: &Thread, short: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = Int::new(short as i32);
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ldc(&self, thread: &Thread, value: LdcArg) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value: Value = match value {
            LdcArg::Int(int) => Int::new(int).into(),
            LdcArg::Float(float) => Float::new(float).into()
        };
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ldc_w(&self, thread: &Thread, value: LdcArg) -> Result<(), ExecError> {
        self.ldc(thread, value)
    }

    pub(super) fn ldc2_w(&self, thread: &Thread, value: Ldc2Arg) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value: Value = match value {
            Ldc2Arg::Long(long) => Long::new(long).into(),
            Ldc2Arg::Double(double) => Double::new(double).into()
        };
        frame.stack().push(value)?;
        thread.runtime().emit_event(VmEvent::OperandStackChange);
        frame.inc_pc();
        Ok(())
    }
}