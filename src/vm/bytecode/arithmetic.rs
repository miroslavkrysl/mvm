use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::error::ExecError;
use crate::vm::types::int::Int;
use crate::vm::types::float::Float;
use crate::vm::types::long::Long;
use crate::vm::types::double::Double;


impl Instruction {
    pub(super) fn iadd(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.add(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ladd(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.add(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fadd(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.add(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dadd(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.add(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn isub(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.sub(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lsub(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.sub(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fsub(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.sub(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dsub(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.sub(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn imul(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.mul(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lmul(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.mul(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fmul(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.mul(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dmul(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.mul(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn idiv(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.div(&value2)?;
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ldiv(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.div(&value2)?;
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fdiv(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.div(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ddiv(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.div(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn irem(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.rem(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lrem(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.rem(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn frem(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Float>()?;
        let value1 = frame.stack().pop::<Float>()?;
        let result = value1.rem(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn drem(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Double>()?;
        let value1 = frame.stack().pop::<Double>()?;
        let result = value1.rem(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ineg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Int>()?;
        let result = value.neg();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lneg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Long>()?;
        let result = value.neg();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn fneg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Float>()?;
        let result = value.neg();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn dneg(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.stack().pop::<Double>()?;
        let result = value.neg();
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ishl(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Int>()?;
        let result = value.shl(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lshl(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Long>()?;
        let result = value.shl(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ishr(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Int>()?;
        let result = value.shr(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lshr(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Long>()?;
        let result = value.shr(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn iushr(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Int>()?;
        let result = value.ushr(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lushr(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let shift = frame.stack().pop::<Int>()?;
        let value = frame.stack().pop::<Long>()?;
        let result = value.ushr(&shift);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn iand(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.and(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn land(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.and(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ior(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.or(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lor(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.or(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn ixor(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Int>()?;
        let value1 = frame.stack().pop::<Int>()?;
        let result = value1.xor(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn lxor(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value2 = frame.stack().pop::<Long>()?;
        let value1 = frame.stack().pop::<Long>()?;
        let result = value1.xor(&value2);
        frame.stack().push(result)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn iinc(&self, thread: &Thread, index: u8, byte: i8) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let value = frame.locals().load::<Int>(index as usize)?;
        let result = value.add(&Int::new(byte as i32));
        frame.locals().store(index as usize, result)?;
        frame.inc_pc();
        Ok(())
    }
}