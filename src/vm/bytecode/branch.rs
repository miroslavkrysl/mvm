use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;
use crate::vm::types::double::Double;
use crate::vm::types::float::Float;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::reference::Reference;


impl Instruction {
    pub(super) fn lcmp(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.cmp(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fcmpl(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.cmpl(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fcmpg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.cmpg(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dcmpl(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.cmpl(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dcmpg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.cmpg(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ifeq(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if value.eq(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn ifne(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if !value.eq(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn iflt(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if value.lt(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn ifge(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if value.ge(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn ifgt(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if value.gt(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn ifle(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        if value.le(&Int::new(0)) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmpeq(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if value1.eq(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmpne(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if !value1.eq(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmplt(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if value1.lt(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmpge(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if value1.ge(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmpgt(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if value1.gt(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_icmple(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        if value1.le(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_acmpeq(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Reference>()?;
        let value1 = frame.stack().pop::<Reference>()?;
        if value1.eq(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn if_acmpne(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Reference>()?;
        let value1 = frame.stack().pop::<Reference>()?;
        if !value1.eq(&value2) {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn goto(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.offset_pc(offset);
        Ok(())
    }

    pub(super) fn ifnull(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        if frame.stack().pop::<Reference>()?.is_null() {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }

    pub(super) fn ifnonnull(&self, thread: &Thread, offset: i16) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        if !frame.stack().pop::<Reference>()?.is_null() {
            frame.offset_pc(offset);
        } else {
            frame.inc_pc();
        }
        Ok(())
    }
}