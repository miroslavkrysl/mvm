use crate::vm::memory::frame_stack::FrameStack;
use std::sync::Arc;
use crate::vm::exec::runtime::Runtime;
use crate::vm::exec::error::{ExecError, InterpreterError};
use crate::vm::instruction::{Instruction, LdcArg, Ldc2Arg};
use crate::vm::memory::frame::Frame;
use crate::vm::types::reference::Reference;
use crate::vm::types::int::Int;
use crate::vm::class::descriptor::{TypeDesc, ReturnDesc};
use crate::vm::types::value::ValueType;
use crate::vm::types::long::Long;
use crate::vm::types::float::Float;
use crate::vm::types::double::Double;
use std::sync::mpsc::{Sender, Receiver};


pub struct Interpreter {
    stack: Arc<FrameStack>,
    runtime: Arc<Runtime>,
}

impl Interpreter {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Interpreter {
            runtime,
            stack: Arc::new(FrameStack::new())
        }
    }

    pub fn next(&self, frame_stack: &FrameStack) -> Result<(), ExecError> {
        let frame = frame_stack.current().unwrap();
        let pc = frame.pc();
        let instruction = frame.method().code().instruction(pc)?;

        match instruction {
            Instruction::NOP => {
                frame.inc_pc();
                runtime.notify()
            },
            Instruction::ICONST_M1 => {
                frame.stack().push(Int::new(-1))?;
                frame.inc_pc();
            },
            Instruction::ACONST_NULL => {
                frame.stack().push(Reference::null())?;
                frame.inc_pc();
            },
            Instruction::ICONST_0 => {
                frame.stack().push(Int::new(0))?;
                frame.inc_pc();
            },
            Instruction::ICONST_1 => {
                frame.stack().push(Int::new(1))?;
                frame.inc_pc();
            },
            Instruction::ICONST_2 => {
                frame.stack().push(Int::new(2))?;
                frame.inc_pc();
            },
            Instruction::ICONST_3 => {
                frame.stack().push(Int::new(3))?;
                frame.inc_pc();
            },
            Instruction::ICONST_4 => {
                frame.stack().push(Int::new(4))?;
                frame.inc_pc();
            },
            Instruction::ICONST_5 => {
                frame.stack().push(Int::new(5))?;
                frame.inc_pc();
            },
            Instruction::LCONST_0 => {
                frame.stack().push(Long::new(0))?;
                frame.inc_pc();
            },
            Instruction::LCONST_1 => {
                frame.stack().push(Long::new(1))?;
                frame.inc_pc();
            },
            Instruction::FCONST_0 => {
                frame.stack().push(Float::new(0.0))?;
                frame.inc_pc();
            },
            Instruction::FCONST_1 => {
                frame.stack().push(Float::new(1.0))?;
                frame.inc_pc();
            },
            Instruction::FCONST_2 => {
                frame.stack().push(Float::new(2.0))?;
                frame.inc_pc();
            },
            Instruction::DCONST_0 => {
                frame.stack().push(Double::new(0.0))?;
                frame.inc_pc();
            },
            Instruction::DCONST_1 => {
                frame.stack().push(Double::new(1.0))?;
                frame.inc_pc();
            },
            Instruction::BIPUSH(byte) => {
                frame.stack().push(Int::new(byte as i32))?;
                frame.inc_pc();
            },
            Instruction::SIPUSH(short) => {
                frame.stack().push(Int::new(short as i32))?;
                frame.inc_pc();
            },
            Instruction::LDC(value) => {
                match value {
                    LdcArg::Int(int) => frame.stack().push(Int::new(int))?,
                    LdcArg::Float(float) => frame.stack().push(Float::new(float))?,
                };
                frame.inc_pc();
            },
            Instruction::LDC_W(value) => {
                match value {
                    LdcArg::Int(int) => frame.stack().push(Int::new(int))?,
                    LdcArg::Float(float) => frame.stack().push(Float::new(float))?,
                };
                frame.inc_pc();
            },
            Instruction::LDC2_W(value) => {
                match value {
                    Ldc2Arg::Long(long) => frame.stack().push(Long::new(long))?,
                    Ldc2Arg::Double(double) => frame.stack().push(Double::new(double))?,
                };
                frame.inc_pc();
            },
            Instruction::ILOAD(index) => {
                let value = frame.locals().load::<Int>(index as usize)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::LLOAD(index) => {
                let value = frame.locals().load::<Int>(index as usize)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::FLOAD(index) => {
                let value = frame.locals().load::<Float>(index as usize)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::DLOAD(index) => {
                let value = frame.locals().load::<Double>(index as usize)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ALOAD(index) => {
                let value = frame.locals().load::<Reference>(index as usize)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ILOAD_0 => {
                let value = frame.locals().load::<Int>(0)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ILOAD_1 => {
                let value = frame.locals().load::<Int>(1)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ILOAD_2 => {
                let value = frame.locals().load::<Int>(2)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ILOAD_3 => {
                let value = frame.locals().load::<Int>(3)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::LLOAD_0 => {
                let value = frame.locals().load::<Long>(0)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::LLOAD_1 => {
                let value = frame.locals().load::<Long>(1)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::LLOAD_2 => {
                let value = frame.locals().load::<Long>(2)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::LLOAD_3 => {
                let value = frame.locals().load::<Long>(3)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::FLOAD_0 => {
                let value = frame.locals().load::<Float>(0)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::FLOAD_1 => {
                let value = frame.locals().load::<Float>(1)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::FLOAD_2 => {
                let value = frame.locals().load::<Float>(2)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::FLOAD_3 => {
                let value = frame.locals().load::<Float>(3)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::DLOAD_0 => {
                let value = frame.locals().load::<Double>(0)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::DLOAD_1 => {
                let value = frame.locals().load::<Double>(1)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::DLOAD_2 => {
                let value = frame.locals().load::<Double>(2)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::DLOAD_3 => {
                let value = frame.locals().load::<Double>(3)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ALOAD_0 => {
                let value = frame.locals().load::<Reference>(0)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ALOAD_1 => {
                let value = frame.locals().load::<Reference>(1)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ALOAD_2 => {
                let value = frame.locals().load::<Reference>(2)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ALOAD_3 => {
                let value = frame.locals().load::<Reference>(3)?;
                frame.stack().push(value)?;
                frame.inc_pc();
            },
            Instruction::ISTORE(index) => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            },
            Instruction::LSTORE(index) => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            },
            Instruction::FSTORE(index) => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            },
            Instruction::DSTORE(index) => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            },
            Instruction::ASTORE(index) => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            },
            Instruction::ISTORE_0 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            },
            Instruction::ISTORE_1 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            },
            Instruction::ISTORE_2 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            },
            Instruction::ISTORE_3 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            },
            Instruction::LSTORE_0 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            },
            Instruction::LSTORE_1 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            },
            Instruction::LSTORE_2 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            },
            Instruction::LSTORE_3 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            },
            Instruction::FSTORE_0 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            },
            Instruction::FSTORE_1 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            },
            Instruction::FSTORE_2 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            },
            Instruction::FSTORE_3 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            },
            Instruction::DSTORE_0 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            },
            Instruction::DSTORE_1 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            },
            Instruction::DSTORE_2 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            },
            Instruction::DSTORE_3 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            },
            Instruction::ASTORE_0 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            },
            Instruction::ASTORE_1 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            },
            Instruction::ASTORE_2 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            },
            Instruction::ASTORE_3 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            },
            Instruction::POP => {
                frame.stack().pop_discard1()?;
                frame.inc_pc();
            },
            Instruction::POP2 => {
                frame.stack().pop_discard2()?;
                frame.inc_pc();
            },
            Instruction::DUP => {
                frame.stack().dup1()?;
                frame.inc_pc();
            },
            Instruction::DUP_X1 => {
                frame.stack().dup1_skip1()?;
                frame.inc_pc();
            },
            Instruction::DUP_X2 => {
                frame.stack().dup1_skip2()?;
                frame.inc_pc();
            },
            Instruction::DUP2 => {
                frame.stack().dup2()?;
                frame.inc_pc();
            },
            Instruction::DUP2_X1 => {
                frame.stack().dup2_skip1()?;
                frame.inc_pc();
            },
            Instruction::DUP2_X2 => {
                frame.stack().dup2_skip2()?;
                frame.inc_pc();
            },
            Instruction::SWAP => {
                frame.stack().swap()?;
                frame.inc_pc();
            },
            Instruction::IADD => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.add(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LADD => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.add(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FADD => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.add(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DADD => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.add(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::ISUB => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.sub(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LSUB => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.sub(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FSUB => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.sub(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DSUB => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.sub(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IMUL => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.mul(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LMUL => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.mul(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FMUL => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.mul(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DMUL => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.mul(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IDIV => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.div(&value2)?;
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LDIV => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.div(&value2)?;
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FDIV => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.div(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DDIV => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.div(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IREM => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.rem(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LREM => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.rem(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FREM => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.rem(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DREM => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.rem(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::INEG => {
                let value = frame.stack().pop::<Int>()?;
                let result = value.neg();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LNEG => {
                let value = frame.stack().pop::<Long>()?;
                let result = value.neg();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FNEG => {
                let value = frame.stack().pop::<Float>()?;
                let result = value.neg();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DNEG => {
                let value = frame.stack().pop::<Double>()?;
                let result = value.neg();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::ISHL => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.shl(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LSHL => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.shl(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::ISHR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.shr(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LSHR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.shr(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IUSHR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.ushr(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LUSHR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.ushr(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IAND => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.and(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LAND => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.and(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IOR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.or(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LOR => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.or(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IXOR => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value1.xor(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LXOR => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.xor(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IINC(index, byte) => {
                let value = frame.locals().load::<Int>(index as usize)?;
                let result = value.add(&Int::new(byte as i32));
                frame.locals().store(index as usize, result)?;
                frame.inc_pc();
            },
            Instruction::I2L => {
                let value = frame.stack().pop::<Int>()?;
                let result = value.to_long();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::I2F => {
                let value = frame.stack().pop::<Int>()?;
                let result = value.to_float();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::I2D => {
                let value = frame.stack().pop::<Int>()?;
                let result = value.to_double();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::L2I => {
                let value = frame.stack().pop::<Long>()?;
                let result = value.to_int();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::L2F => {
                let value = frame.stack().pop::<Long>()?;
                let result = value.to_float();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::L2D => {
                let value = frame.stack().pop::<Long>()?;
                let result = value.to_double();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::F2I => {
                let value = frame.stack().pop::<Float>()?;
                let result = value.to_int();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::F2L => {
                let value = frame.stack().pop::<Float>()?;
                let result = value.to_long();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::F2D => {
                let value = frame.stack().pop::<Float>()?;
                let result = value.to_double();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::D2I => {
                let value = frame.stack().pop::<Double>()?;
                let result = value.to_float();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::D2L => {
                let value = frame.stack().pop::<Double>()?;
                let result = value.to_float();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::D2F => {
                let value = frame.stack().pop::<Double>()?;
                let result = value.to_float();
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::LCMP => {
                let value2 = frame.stack().pop::<Long>()?;
                let value1 = frame.stack().pop::<Long>()?;
                let result = value1.cmp(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FCMPL => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.cmpl(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::FCMPG => {
                let value2 = frame.stack().pop::<Float>()?;
                let value1 = frame.stack().pop::<Float>()?;
                let result = value1.cmpg(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DCMPL => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.cmpl(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::DCMPG => {
                let value2 = frame.stack().pop::<Double>()?;
                let value1 = frame.stack().pop::<Double>()?;
                let result = value1.cmpg(&value2);
                frame.stack().push(result)?;
                frame.inc_pc();
            },
            Instruction::IFEQ(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if value.eq(&Int::new(0)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IFNE(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if !value.eq(&Int::new(02)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IFLT(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if value.lt(&Int::new(0)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IFGE(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if value.ge(&Int::new(0)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IFGT(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if value.gt(&Int::new(0)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IFLE(offset) => {
                let value = frame.stack().pop::<Int>()?;
                if value.le(&Int::new(0)) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPEQ(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if value1.eq(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPNE(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if !value1.eq(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPLT(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if value1.lt(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPGE(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if value1.ge(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPGT(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if value1.gt(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ICMPLE(offset) => {
                let value2 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                if value1.le(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ACMPEQ(offset) => {
                let value2 = frame.stack().pop::<Reference>()?;
                let value1 = frame.stack().pop::<Reference>()?;
                if value1.eq(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::IF_ACMPNE(offset) => {
                let value2 = frame.stack().pop::<Reference>()?;
                let value1 = frame.stack().pop::<Reference>()?;
                if !value1.eq(&value2) {
                    frame.offset_pc(offset);
                } else {
                    frame.inc_pc();
                }
            },
            Instruction::GOTO(offset) => {
                frame.offset_pc(offset);
            },
            Instruction::IFNULL(offset) => {
                if frame.stack().pop::<Reference>()?.is_null() {
                    frame.offset_pc(offset);
                }
            },
            Instruction::IFNONNULL(offset) => {
                if !frame.stack().pop::<Reference>()?.is_null() {
                    frame.offset_pc(offset);
                }
            },
            Instruction::IRETURN => {
                let expected_type = frame.method().signature().return_desc();

                if *expected_type != ReturnDesc::NonVoid(TypeDesc::Int) {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Int
                    })
                }

                let value = frame.stack().pop::<Int>()?;
                self.stack.pop();
                self.stack.current().unwrap().stack().push(value)?;
            },
            Instruction::LRETURN => {
                let expected_type = frame.method().signature().return_desc();

                if *expected_type != ReturnDesc::NonVoid(TypeDesc::Long) {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Long
                    })
                }

                let value = frame.stack().pop::<Long>()?;
                self.stack.pop();
                self.stack.current().unwrap().stack().push(value)?;
            },
            Instruction::FRETURN => {
                let expected_type = frame.method().signature().return_desc();

                if *expected_type != ReturnDesc::NonVoid(TypeDesc::Float) {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Float
                    })
                }

                let value = frame.stack().pop::<Float>()?;
                self.stack.pop();
                self.stack.current().unwrap().stack().push(value)?;
            },
            Instruction::DRETURN => {
                let expected_type = frame.method().signature().return_desc();

                if *expected_type != ReturnDesc::NonVoid(TypeDesc::Double) {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Double
                    })
                }

                let value = frame.stack().pop::<Double>()?;
                self.stack.pop();
                self.stack.current().unwrap().stack().push(value)?;
            },
            Instruction::ARETURN => {
                let expected_type = frame.method().signature().return_desc();

                if let ReturnDesc::NonVoid(TypeDesc::Reference(class_name)) = expected_type {
                    let value = frame.stack().pop::<Reference>()?;
                    let instance = value.as_instance().unwrap();

                    if !value.is_null() && *instance.class().name() != *class_name {
                        return Err(ExecError::InvalidReturnReference {
                            expected: class_name.clone(),
                            found: instance.class().name().clone()
                        })
                    }

                    self.stack.pop();
                    self.stack.current().unwrap().stack().push(value)?;
                } else {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Reference
                    })
                }
            },
            Instruction::RETURN => {
                let expected_type = frame.method().signature().return_desc();

                if *expected_type != ReturnDesc::NonVoid(TypeDesc::Int) {
                    return Err(ExecError::InvalidReturnType {
                        expected: expected_type.clone(),
                        called: ValueType::Int
                    })
                }

                let value = frame.stack().pop::<Int>()?;
                self.stack.pop();
                self.stack.current().unwrap().stack().push(value)?;
            },
            Instruction::GETSTATIC(field_ref) => {
                let class = self.runtime.class(field_ref.class_name())?;
                let value = class.static_field_value(field_ref.signature())?;
                frame.stack().push_value(value)?;
                frame.inc_pc();
            },
            Instruction::PUTSTATIC(field_ref) => {
                let class = self.runtime.class(field_ref.class_name())?;
                let value = frame.stack().pop_value()?;
                class.set_static_field_value(field_ref.signature(), value)?;
                frame.inc_pc();
            },
            Instruction::GETFIELD(field_ref) => {
                let class = self.runtime.class(field_ref.class_name())?;
                let instance = frame.stack().pop::<Reference>()?.to_instance()?;
                let value = class.instance_field_value(&instance, field_ref.signature())?;
                frame.stack().push_value(value)?;
                frame.inc_pc();
            },
            Instruction::PUTFIELD(field_ref) => {
                let class = self.runtime.class(field_ref.class_name())?;
                let value = frame.stack().pop_value()?;
                let instance = frame.stack().pop::<Reference>()?.to_instance()?;
                class.set_instance_field_value(&instance, field_ref.signature(), value)?;
                frame.inc_pc();
            },
            Instruction::INVOKEVIRTUAL(method_ref) => {
                let class = self.runtime.class(method_ref.class_name())?;
                let method = class.instance_method(method_ref.signature())?;
                let next_frame = Frame::new_from_call(method.clone(), frame.stack())?;
                self.stack.push(next_frame);
                frame.inc_pc();
            },
            Instruction::INVOKESTATIC(method_ref) => {
                let class = self.runtime.class(method_ref.class_name())?;
                let method = class.static_method(method_ref.signature())?;
                let next_frame = Frame::new_from_call(method.clone(), frame.stack())?;
                self.stack.push(next_frame);
                frame.inc_pc();
            },
            Instruction::NEW(class_name) => {
                let reference = self.runtime.create_instance(class_name)?;
                frame.stack().push(reference)?;
                frame.inc_pc();
            },
        }

        Ok(())
    }
}

impl Interpreter {
    fn nop(&self) -> Result<(), InterpreterError> {
        self.stack.current().
        Ok(())
    }

    fn nop(&self) -> Result<(), InterpreterError> {
        Ok(())
    }
}