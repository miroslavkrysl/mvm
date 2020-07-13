use std::sync::Arc;

use crate::vm::instruction::Instruction;
use crate::vm::types::{Double, Float, Int, Long, Reference};
use crate::vm::exec::ExecError;
use crate::vm::memory::{Frame, FrameStack};
use std::thread::JoinHandle;
use std::thread;
use crate::vm::class::Method;

pub struct Thread {
    join_handle: Option<JoinHandle<()>>,
    stack: FrameStack
}

impl Thread {
    pub fn new(Callee: Callee) -> Self {
        Thread {
            join_handle: None,
            stack: FrameStack::new()
        }
    }

    /// Start executing this thread concurrently.
    pub fn start(self: Arc<Self>) {
        let this = self.clone();

        let join_handle = thread::spawn(move || {
           self.
        });
    }

    pub fn run(&mut self, method: Arc<Method>, args: Vec<JvmValue>) -> Result<(), ()>{
        self.stack.push(Frame::for_method(method, args));

        // method loop
        loop {
            let frame = self.stack.current_mut().unwrap();
            let code = frame.method().code().unwrap();

            // instruction loop
            let result = loop {

                let pc = frame.pc();
                let result = self.execute(pc);

                Ok(pc)
            }
        }
    }

    fn execute_instruction(&mut self, mut frame: Frame) -> Result<(), ExecError> {
        let instruction: Instruction = Instruction::ARETURN;

        match instruction {
            Instruction::NOP => {
                frame.inc_pc();
            }
            Instruction::ACONST_NULL => {
                frame.stack_mut().push(Reference::null())?;
                frame.inc_pc();
            }
            Instruction::ICONST_M1 => {
                frame.stack().push(Int::new(-1))?;
                frame.inc_pc();
            }
            Instruction::ICONST_0 => {
                frame.stack().push(Int::new(0))?;
                frame.inc_pc();
            }
            Instruction::ICONST_1 => {
                frame.stack().push(Int::new(1))?;
                frame.inc_pc();
            }
            Instruction::ICONST_2 => {
                frame.stack().push(Int::new(2))?;
                frame.inc_pc();
            }
            Instruction::ICONST_3 => {
                frame.stack().push(Int::new(3));
                frame.inc_pc();
            }
            Instruction::ICONST_4 => {
                frame.stack().push(Int::new(4));
                frame.inc_pc();
            }
            Instruction::ICONST_5 => {
                frame.stack().push(Int::new(5));
                frame.inc_pc();
            }
            Instruction::LCONST_0 => {
                frame.stack().push(Long::new(0));
                frame.inc_pc();
            }
            Instruction::LCONST_1 => {
                frame.stack().push(Long::new(1));
                frame.inc_pc();
            }
            Instruction::FCONST_0 => {
                frame.stack().push(Float::new(0.0));
                frame.inc_pc();
            }
            Instruction::FCONST_1 => {
                frame.stack().push(Float::new(1.0));
                frame.inc_pc();
            }
            Instruction::FCONST_2 => {
                frame.stack().push(Float::new(2.0));
                frame.inc_pc();
            }
            Instruction::DCONST_0 => {
                frame.stack().push(Double::new(0.0));
                frame.inc_pc();
            }
            Instruction::DCONST_1 => {
                frame.stack().push(Double::new(1.0));
                frame.inc_pc();
            }
            Instruction::BIPUSH { value } => {
                frame.stack().push(Int::new(value as i32));
                frame.inc_pc();
            }
            Instruction::SIPUSH { value } => {
                frame.stack().push(Int::new(value as i32));
                frame.inc_pc();
            }
            // Instruction::LDC { .. } => {}
            // Instruction::LDC_W { .. } => {}
            // Instruction::LDC2_W { .. } => {}
            Instruction::ILOAD { index } => {
                let value = frame.locals().load::<Int>(index as usize)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::LLOAD { index } => {
                let value = frame.locals().load::<Long>(index as usize)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::FLOAD { index } => {
                let value = frame.locals().load::<Float>(index as usize)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::DLOAD { index } => {
                let value = frame.locals().load::<Double>(index as usize)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ALOAD { index } => {
                let value = frame.locals().load::<Reference>(index as usize)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ILOAD_0 => {
                let value = frame.locals().load::<Int>(0)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ILOAD_1 => {
                let value = frame.locals().load::<Int>(1)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ILOAD_2 => {
                let value = frame.locals().load::<Int>(2)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ILOAD_3 => {
                let value = frame.locals().load::<Int>(3)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::LLOAD_0 => {
                let value = frame.locals().load::<Long>(0)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::LLOAD_1 => {
                let value = frame.locals().load::<Long>(1)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::LLOAD_2 => {
                let value = frame.locals().load::<Long>(2)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::LLOAD_3 => {
                let value = frame.locals().load::<Long>(3)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::FLOAD_0 => {
                let value = frame.locals().load::<Float>(0)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::FLOAD_1 => {
                let value = frame.locals().load::<Float>(1)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::FLOAD_2 => {
                let value = frame.locals().load::<Float>(2)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::FLOAD_3 => {
                let value = frame.locals().load::<Float>(3)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::DLOAD_0 => {
                let value = frame.locals().load::<Double>(0)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::DLOAD_1 => {
                let value = frame.locals().load::<Double>(1)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::DLOAD_2 => {
                let value = frame.locals().load::<Double>(2)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::DLOAD_3 => {
                let value = frame.locals().load::<Double>(3)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ALOAD_0 => {
                let value = frame.locals().load::<Reference>(0)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ALOAD_1 => {
                let value = frame.locals().load::<Reference>(1)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ALOAD_2 => {
                let value = frame.locals().load::<Reference>(2)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            Instruction::ALOAD_3 => {
                let value = frame.locals().load::<Reference>(3)?;
                frame.stack().push(value);
                frame.inc_pc();
            }
            // Instruction::IALOAD => {}
            // Instruction::LALOAD => {}
            // Instruction::FALOAD => {}
            // Instruction::DALOAD => {}
            // Instruction::AALOAD => {}
            // Instruction::BALOAD => {}
            // Instruction::CALOAD => {}
            // Instruction::SALOAD => {}
            Instruction::ISTORE { index } => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            }
            Instruction::LSTORE { index } => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            }
            Instruction::FSTORE { index } => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            }
            Instruction::DSTORE { index } => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            }
            Instruction::ASTORE { index } => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(index as usize, value)?;
                frame.inc_pc();
            }
            Instruction::ISTORE_0 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            }
            Instruction::ISTORE_1 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            }
            Instruction::ISTORE_2 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            }
            Instruction::ISTORE_3 => {
                let value = frame.stack().pop::<Int>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            }
            Instruction::LSTORE_0 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            }
            Instruction::LSTORE_1 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            }
            Instruction::LSTORE_2 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            }
            Instruction::LSTORE_3 => {
                let value = frame.stack().pop::<Long>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            }
            Instruction::FSTORE_0 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            }
            Instruction::FSTORE_1 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            }
            Instruction::FSTORE_2 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            }
            Instruction::FSTORE_3 => {
                let value = frame.stack().pop::<Float>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            }
            Instruction::DSTORE_0 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            }
            Instruction::DSTORE_1 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            }
            Instruction::DSTORE_2 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            }
            Instruction::DSTORE_3 => {
                let value = frame.stack().pop::<Double>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            }
            Instruction::ASTORE_0 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(0, value)?;
                frame.inc_pc();
            }
            Instruction::ASTORE_1 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(1, value)?;
                frame.inc_pc();
            }
            Instruction::ASTORE_2 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(2, value)?;
                frame.inc_pc();
            }
            Instruction::ASTORE_3 => {
                let value = frame.stack().pop::<Reference>()?;
                frame.locals().store(3, value)?;
                frame.inc_pc();
            }
            // Instruction::IASTORE => {}
            // Instruction::LASTORE => {}
            // Instruction::FASTORE => {}
            // Instruction::DASTORE => {}
            // Instruction::AASTORE => {}
            // Instruction::BASTORE => {}
            // Instruction::CASTORE => {}
            // Instruction::SASTORE => {}
            Instruction::POP => {
                frame.stack().pop_discard1()?;
                frame.inc_pc();
            }
            Instruction::POP2 => {
                frame.stack().pop_discard2()?;
                frame.inc_pc();
            }
            Instruction::DUP => {
                frame.stack().dup1()?;
                frame.inc_pc();
            }
            Instruction::DUP_X1 => {
                frame.stack().dup1_skip1()?;
                frame.inc_pc();
            }
            Instruction::DUP_X2 => {
                frame.stack().dup1_skip2()?;
                frame.inc_pc();
            }
            Instruction::DUP2 => {
                frame.stack().dup2()?;
                frame.inc_pc();
            }
            Instruction::DUP2_X1 => {
                frame.stack().dup2_skip1()?;
                frame.inc_pc();
            }
            Instruction::DUP2_X2 => {
                frame.stack().dup2_skip2()?;
                frame.inc_pc();
            }
            Instruction::SWAP => {
                frame.stack().swap()?;
                frame.inc_pc();
            }
            Instruction::IADD => {
                let value0 = frame.stack().pop::<Int>()?;
                let value1 = frame.stack().pop::<Int>()?;
                let result = value0.add(&value1);
                frame.stack().push(result);
                frame.inc_pc();
            }
            // Instruction::LADD => {}
            // Instruction::FADD => {}
            // Instruction::DADD => {}
            // Instruction::ISUB => {}
            // Instruction::LSUB => {}
            // Instruction::FSUB => {}
            // Instruction::DSUB => {}
            // Instruction::IMUL => {}
            // Instruction::LMUL => {}
            // Instruction::FMUL => {}
            // Instruction::DMUL => {}
            // Instruction::IDIV => {}
            // Instruction::LDIV => {}
            // Instruction::FDIV => {}
            // Instruction::DDIV => {}
            // Instruction::IREM => {}
            // Instruction::LREM => {}
            // Instruction::FREM => {}
            // Instruction::DREM => {}
            // Instruction::INEG => {}
            // Instruction::LNEG => {}
            // Instruction::FNEG => {}
            // Instruction::DNEG => {}
            // Instruction::ISHL => {}
            // Instruction::LSHL => {}
            // Instruction::ISHR => {}
            // Instruction::LSHR => {}
            // Instruction::IUSHR => {}
            // Instruction::LUSHR => {}
            // Instruction::IAND => {}
            // Instruction::LAND => {}
            // Instruction::IOR => {}
            // Instruction::LOR => {}
            // Instruction::IXOR => {}
            // Instruction::LXOR => {}
            // Instruction::IINC { .. } => {}
            // Instruction::I2L => {}
            // Instruction::I2F => {}
            // Instruction::I2D => {}
            // Instruction::L2I => {}
            // Instruction::L2F => {}
            // Instruction::L2D => {}
            // Instruction::F2I => {}
            // Instruction::F2L => {}
            // Instruction::F2D => {}
            // Instruction::D2I => {}
            // Instruction::D2L => {}
            // Instruction::D2F => {}
            // Instruction::I2B => {}
            // Instruction::I2C => {}
            // Instruction::I2S => {}
            // Instruction::LCMP => {}
            // Instruction::FCMPL => {}
            // Instruction::FCMPG => {}
            // Instruction::DCMPL => {}
            // Instruction::DCMPG => {}
            // Instruction::IFEQ { .. } => {}
            // Instruction::IFNE { .. } => {}
            // Instruction::IFLT { .. } => {}
            // Instruction::IFGE { .. } => {}
            // Instruction::IFGT { .. } => {}
            // Instruction::IFLE { .. } => {}
            // Instruction::IF_ICMPEQ { .. } => {}
            // Instruction::IF_ICMPNE { .. } => {}
            // Instruction::IF_ICMPLT { .. } => {}
            // Instruction::IF_ICMPGE { .. } => {}
            // Instruction::IF_ICMPGT { .. } => {}
            // Instruction::IF_ICMPLE { .. } => {}
            // Instruction::IF_ACMPEQ { .. } => {}
            // Instruction::IF_ACMPNE { .. } => {}
            // Instruction::GOTO { .. } => {}
            // Instruction::IRETURN => {}
            // Instruction::LRETURN => {}
            // Instruction::FRETURN => {}
            // Instruction::DRETURN => {}
            // Instruction::ARETURN => {}
            // Instruction::RETURN => {}
            // Instruction::GETSTATIC { .. } => {}
            // Instruction::PUTSTATIC { .. } => {}
            Instruction::GETFIELD { index } => {
                // let field_ref = frame.class().constant_pool().load_constant(index as usize)?;
                // let object = frame.pop_operand();
                // frame.push_operand(value);
            }
            // Instruction::PUTFIELD { .. } => {}
            // Instruction::INVOKEVIRTUAL { .. } => {}
            // Instruction::INVOKESPECIAL { .. } => {}
            // Instruction::INVOKESTATIC { .. } => {}
            // Instruction::NEW { .. } => {}
            // Instruction::NEWARRAY { .. } => {}
            // Instruction::ANEWARRAY { .. } => {}
            // Instruction::ARRAYLENGTH => {}
            // Instruction::INSTANCEOF { .. } => {}
            // Instruction::IFNULL { .. } => {}
            // Instruction::IFNONNULL { .. } => {}
            // Instruction::GOTO_W { .. } => {}
            _ => unimplemented!()
        };

        Ok(())
    }
}