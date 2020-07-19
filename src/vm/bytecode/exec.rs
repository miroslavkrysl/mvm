use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::error::ExecError;
use crate::vm::exec::thread::Thread;


impl Instruction {
    pub fn execute(&self, thread: &Thread) -> Result<(), ExecError> {
        match self {
            Instruction::NOP => self.nop(thread)?,
            Instruction::ICONST_M1 => self.iconst_m1(thread)?,
            Instruction::ACONST_NULL => self.aconst_null(thread)?,
            Instruction::ICONST_0 => self.iconst_0(thread)?,
            Instruction::ICONST_1 => self.iconst_1(thread)?,
            Instruction::ICONST_2 => self.iconst_2(thread)?,
            Instruction::ICONST_3 => self.iconst_3(thread)?,
            Instruction::ICONST_4 => self.iconst_4(thread)?,
            Instruction::ICONST_5 => self.iconst_5(thread)?,
            Instruction::LCONST_0 => self.lconst_0(thread)?,
            Instruction::LCONST_1 => self.lconst_1(thread)?,
            Instruction::FCONST_0 => self.fconst_0(thread)?,
            Instruction::FCONST_1 => self.fconst_1(thread)?,
            Instruction::FCONST_2 => self.fconst_2(thread)?,
            Instruction::DCONST_0 => self.dconst_0(thread)?,
            Instruction::DCONST_1 => self.dconst_1(thread)?,
            Instruction::BIPUSH(byte) => self.bipush(thread, *byte)?,
            Instruction::SIPUSH(short) => self.sipush(thread, *short)?,
            Instruction::LDC(value) => self.ldc(thread, *value)?,
            Instruction::LDC_W(value) => self.ldc_w(thread, *value)?,
            Instruction::LDC2_W(value) => self.ldc2_w(thread, *value)?,
            Instruction::ILOAD(index) => self.iload(thread, *index)?,
            Instruction::LLOAD(index) => self.lload(thread, *index)?,
            Instruction::FLOAD(index) => self.fload(thread, *index)?,
            Instruction::DLOAD(index) => self.dload(thread, *index)?,
            Instruction::ALOAD(index) => self.aload(thread, *index)?,
            Instruction::ILOAD_0 => self.iload_0(thread)?,
            Instruction::ILOAD_1 => self.iload_1(thread)?,
            Instruction::ILOAD_2 => self.iload_2(thread)?,
            Instruction::ILOAD_3 => self.iload_3(thread)?,
            Instruction::LLOAD_0 => self.lload_0(thread)?,
            Instruction::LLOAD_1 => self.lload_1(thread)?,
            Instruction::LLOAD_2 => self.lload_2(thread)?,
            Instruction::LLOAD_3 => self.lload_3(thread)?,
            Instruction::FLOAD_0 => self.fload_0(thread)?,
            Instruction::FLOAD_1 => self.fload_1(thread)?,
            Instruction::FLOAD_2 => self.fload_2(thread)?,
            Instruction::FLOAD_3 => self.fload_3(thread)?,
            Instruction::DLOAD_0 => self.dload_0(thread)?,
            Instruction::DLOAD_1 => self.dload_1(thread)?,
            Instruction::DLOAD_2 => self.dload_2(thread)?,
            Instruction::DLOAD_3 => self.dload_3(thread)?,
            Instruction::ALOAD_0 => self.aload_0(thread)?,
            Instruction::ALOAD_1 => self.aload_1(thread)?,
            Instruction::ALOAD_2 => self.aload_2(thread)?,
            Instruction::ALOAD_3 => self.aload_3(thread)?,
            Instruction::ISTORE(index) => self.istore(thread, *index)?,
            Instruction::LSTORE(index) => self.lstore(thread, *index)?,
            Instruction::FSTORE(index) => self.fstore(thread, *index)?,
            Instruction::DSTORE(index) => self.dstore(thread, *index)?,
            Instruction::ASTORE(index) => self.astore(thread, *index)?,
            Instruction::ISTORE_0 => self.istore_0(thread)?,
            Instruction::ISTORE_1 => self.istore_1(thread)?,
            Instruction::ISTORE_2 => self.istore_2(thread)?,
            Instruction::ISTORE_3 => self.istore_3(thread)?,
            Instruction::LSTORE_0 => self.lstore_0(thread)?,
            Instruction::LSTORE_1 => self.lstore_1(thread)?,
            Instruction::LSTORE_2 => self.lstore_2(thread)?,
            Instruction::LSTORE_3 => self.lstore_3(thread)?,
            Instruction::FSTORE_0 => self.fstore_0(thread)?,
            Instruction::FSTORE_1 => self.fstore_1(thread)?,
            Instruction::FSTORE_2 => self.fstore_2(thread)?,
            Instruction::FSTORE_3 => self.fstore_3(thread)?,
            Instruction::DSTORE_0 => self.dstore_0(thread)?,
            Instruction::DSTORE_1 => self.dstore_1(thread)?,
            Instruction::DSTORE_2 => self.dstore_2(thread)?,
            Instruction::DSTORE_3 => self.dstore_3(thread)?,
            Instruction::ASTORE_0 => self.astore_0(thread)?,
            Instruction::ASTORE_1 => self.astore_1(thread)?,
            Instruction::ASTORE_2 => self.astore_2(thread)?,
            Instruction::ASTORE_3 => self.astore_3(thread)?,
            Instruction::POP => self.pop(thread)?,
            Instruction::POP2 => self.pop2(thread)?,
            Instruction::DUP => self.dup(thread)?,
            Instruction::DUP_X1 => self.dup_x1(thread)?,
            Instruction::DUP_X2 => self.dup_x2(thread)?,
            Instruction::DUP2 => self.dup2(thread)?,
            Instruction::DUP2_X1 => self.dup2_x1(thread)?,
            Instruction::DUP2_X2 => self.dup2_x2(thread)?,
            Instruction::SWAP => self.swap(thread)?,
            Instruction::IADD => self.iadd(thread)?,
            Instruction::LADD => self.ladd(thread)?,
            Instruction::FADD => self.fadd(thread)?,
            Instruction::DADD => self.dadd(thread)?,
            Instruction::ISUB => self.isub(thread)?,
            Instruction::LSUB => self.lsub(thread)?,
            Instruction::FSUB => self.fsub(thread)?,
            Instruction::DSUB => self.dsub(thread)?,
            Instruction::IMUL => self.imul(thread)?,
            Instruction::LMUL => self.lmul(thread)?,
            Instruction::FMUL => self.fmul(thread)?,
            Instruction::DMUL => self.dmul(thread)?,
            Instruction::IDIV => self.idiv(thread)?,
            Instruction::LDIV => self.ldiv(thread)?,
            Instruction::FDIV => self.fdiv(thread)?,
            Instruction::DDIV => self.ddiv(thread)?,
            Instruction::IREM => self.irem(thread)?,
            Instruction::LREM => self.lrem(thread)?,
            Instruction::FREM => self.frem(thread)?,
            Instruction::DREM => self.drem(thread)?,
            Instruction::INEG => self.ineg(thread)?,
            Instruction::LNEG => self.lneg(thread)?,
            Instruction::FNEG => self.fneg(thread)?,
            Instruction::DNEG => self.dneg(thread)?,
            Instruction::ISHL => self.ishl(thread)?,
            Instruction::LSHL => self.lshl(thread)?,
            Instruction::ISHR => self.ishr(thread)?,
            Instruction::LSHR => self.lshr(thread)?,
            Instruction::IUSHR => self.iushr(thread)?,
            Instruction::LUSHR => self.lushr(thread)?,
            Instruction::IAND => self.iand(thread)?,
            Instruction::LAND => self.land(thread)?,
            Instruction::IOR => self.ior(thread)?,
            Instruction::LOR => self.lor(thread)?,
            Instruction::IXOR => self.ixor(thread)?,
            Instruction::LXOR => self.lxor(thread)?,
            Instruction::IINC(index, byte) => self.iinc(thread, *index, *byte)?,
            Instruction::I2L => self.i2l(thread)?,
            Instruction::I2F => self.i2f(thread)?,
            Instruction::I2D => self.i2d(thread)?,
            Instruction::L2I => self.l2i(thread)?,
            Instruction::L2F => self.l2f(thread)?,
            Instruction::L2D => self.l2d(thread)?,
            Instruction::F2I => self.f2i(thread)?,
            Instruction::F2L => self.f2l(thread)?,
            Instruction::F2D => self.f2d(thread)?,
            Instruction::D2I => self.d2i(thread)?,
            Instruction::D2L => self.d2l(thread)?,
            Instruction::D2F => self.d2f(thread)?,
            Instruction::LCMP => self.lcmp(thread)?,
            Instruction::FCMPL => self.fcmpl(thread)?,
            Instruction::FCMPG => self.fcmpg(thread)?,
            Instruction::DCMPL => self.dcmpl(thread)?,
            Instruction::DCMPG => self.dcmpg(thread)?,
            Instruction::IFEQ(offset) => self.ifeq(thread, *offset)?,
            Instruction::IFNE(offset) => self.ifne(thread, *offset)?,
            Instruction::IFLT(offset) => self.iflt(thread, *offset)?,
            Instruction::IFGE(offset) => self.ifge(thread, *offset)?,
            Instruction::IFGT(offset) => self.ifgt(thread, *offset)?,
            Instruction::IFLE(offset) => self.ifle(thread, *offset)?,
            Instruction::IF_ICMPEQ(offset) => self.if_icmpeq(thread, *offset)?,
            Instruction::IF_ICMPNE(offset) => self.if_icmpne(thread, *offset)?,
            Instruction::IF_ICMPLT(offset) => self.if_icmplt(thread, *offset)?,
            Instruction::IF_ICMPGE(offset) => self.if_icmpge(thread, *offset)?,
            Instruction::IF_ICMPGT(offset) => self.if_icmpgt(thread, *offset)?,
            Instruction::IF_ICMPLE(offset) => self.if_icmple(thread, *offset)?,
            Instruction::IF_ACMPEQ(offset) => self.if_acmpeq(thread, *offset)?,
            Instruction::IF_ACMPNE(offset) => self.if_acmpne(thread, *offset)?,
            Instruction::GOTO(offset) => self.goto(thread, *offset)?,
            Instruction::IFNULL(offset) => self.ifnull(thread, *offset)?,
            Instruction::IFNONNULL(offset) => self.ifnonnull(thread, *offset)?,
            Instruction::IRETURN => self.ireturn(thread)?,
            Instruction::LRETURN => self.lreturn(thread)?,
            Instruction::FRETURN => self.freturn(thread)?,
            Instruction::DRETURN => self.dreturn(thread)?,
            Instruction::ARETURN => self.areturn(thread)?,
            Instruction::RETURN => self.vreturn(thread)?,
            Instruction::GETSTATIC(field_ref) => self.getstatic(thread, field_ref)?,
            Instruction::PUTSTATIC(field_ref) => self.putstatic(thread, field_ref)?,
            Instruction::GETFIELD(field_ref) => self.getfield(thread, field_ref)?,
            Instruction::PUTFIELD(field_ref) => self.putfield(thread, field_ref)?,
            Instruction::INVOKEVIRTUAL(method_ref) => self.invokevirtual(thread, method_ref)?,
            Instruction::INVOKESTATIC(method_ref) => self.invokestatic(thread, method_ref)?,
            Instruction::NEW(class_name) => self.new(thread, class_name)?,
            _ => unreachable!()
        }
        Ok(())
    }
}


impl Instruction {
    pub(super) fn nop(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        frame.inc_pc();
        Ok(())
    }
}