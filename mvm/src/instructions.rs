#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ArrayType {
    Boolean,
    Char,
    Float,
    Double,
    Byte,
    Short,
    Int,
    Long,
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP,
    ACONST_NULL,
    ICONST_M1,
    ICONST_0,
    ICONST_1,
    ICONST_2,
    ICONST_3,
    ICONST_4,
    ICONST_5,
    LCONST_0,
    LCONST_1,
    FCONST_0,
    FCONST_1,
    FCONST_2,
    DCONST_0,
    DCONST_1,
    BIPUSH {
        value: i8
    },
    SIPUSH {
        value: i16
    },
    LDC {
        index: u8
    },
    LDC_W {
        index: u16
    },
    LDC2_W {
        index: u16
    },
    ILOAD {
        index: u8
    },
    LLOAD {
        index: u8
    },
    FLOAD {
        index: u8
    },
    DLOAD {
        index: u8
    },
    ALOAD {
        index: u8
    },
    ILOAD_0,
    ILOAD_1,
    ILOAD_2,
    ILOAD_3,
    LLOAD_0,
    LLOAD_1,
    LLOAD_2,
    LLOAD_3,
    FLOAD_0,
    FLOAD_1,
    FLOAD_2,
    FLOAD_3,
    DLOAD_0,
    DLOAD_1,
    DLOAD_2,
    DLOAD_3,
    ALOAD_0,
    ALOAD_1,
    ALOAD_2,
    ALOAD_3,
    IALOAD,
    LALOAD,
    FALOAD,
    DALOAD,
    AALOAD,
    BALOAD,
    CALOAD,
    SALOAD,
    ISTORE {
        index: u8
    },
    LSTORE {
        index: u8
    },
    FSTORE {
        index: u8
    },
    DSTORE {
        index: u8
    },
    ASTORE {
        index: u8
    },
    ISTORE_0,
    ISTORE_1,
    ISTORE_2,
    ISTORE_3,
    LSTORE_0,
    LSTORE_1,
    LSTORE_2,
    LSTORE_3,
    FSTORE_0,
    FSTORE_1,
    FSTORE_2,
    FSTORE_3,
    DSTORE_0,
    DSTORE_1,
    DSTORE_2,
    DSTORE_3,
    ASTORE_0,
    ASTORE_1,
    ASTORE_2,
    ASTORE_3,
    IASTORE,
    LASTORE,
    FASTORE,
    DASTORE,
    AASTORE,
    BASTORE,
    CASTORE,
    SASTORE,
    POP,
    POP2,
    DUP,
    DUP_X1,
    DUP_X2,
    DUP2,
    DUP2_X1,
    DUP2_X2,
    SWAP,
    IADD,
    LADD,
    FADD,
    DADD,
    ISUB,
    LSUB,
    FSUB,
    DSUB,
    IMUL,
    LMUL,
    FMUL,
    DMUL,
    IDIV,
    LDIV,
    FDIV,
    DDIV,
    IREM,
    LREM,
    FREM,
    DREM,
    INEG,
    LNEG,
    FNEG,
    DNEG,
    ISHL,
    LSHL,
    ISHR,
    LSHR,
    IUSHR,
    LUSHR,
    IAND,
    LAND,
    IOR,
    LOR,
    IXOR,
    LXOR,
    IINC {
        index: u8,
        value: i8,
    },
    I2L,
    I2F,
    I2D,
    L2I,
    L2F,
    L2D,
    F2I,
    F2L,
    F2D,
    D2I,
    D2L,
    D2F,
    I2B,
    I2C,
    I2S,
    LCMP,
    FCMPL,
    FCMPG,
    DCMPL,
    DCMPG,
    IFEQ {
        offset: i16
    },
    IFNE {
        offset: i16
    },
    IFLT {
        offset: i16
    },
    IFGE {
        offset: i16
    },
    IFGT {
        offset: i16
    },
    IFLE {
        offset: i16
    },

    IF_ICMPEQ {
        offset: i16
    },
    IF_ICMPNE {
        offset: i16
    },
    IF_ICMPLT {
        offset: i16
    },
    IF_ICMPGE {
        offset: i16
    },
    IF_ICMPGT {
        offset: i16
    },
    IF_ICMPLE {
        offset: i16
    },
    IF_ACMPEQ {
        offset: i16
    },
    IF_ACMPNE {
        offset: i16
    },
    GOTO {
        offset: i16
    },

    // Deprecated:
    // JSR,
    // RET,

    // Todo:
    // TABLESWITCH,
    // LOOKUPSWITCH,

    IRETURN,
    LRETURN,
    FRETURN,
    DRETURN,
    ARETURN,
    RETURN,
    GETSTATIC {
        index: u16
    },
    PUTSTATIC {
        index: u16
    },
    GETFIELD {
        index: u16
    },
    PUTFIELD {
        index: u16
    },
    INVOKEVIRTUAL {
        index: u16
    },
    INVOKESPECIAL {
        index: u16
    },
    INVOKESTATIC {
        index: u16
    },

    // Todo:
    // INVOKEINTERFACE,

    // Todo:
    // INVOKEDYNAMIC,

    NEW {
        index: u16
    },
    NEWARRAY {
        array_type: ArrayType
    },
    ANEWARRAY {
        index: u16
    },
    ARRAYLENGTH,

    // Todo:
    // ATHROW,

    // Todo:
    // CHECKCAST {
    //     index: u16
    // },

    INSTANCEOF {
        index: u16
    },

    // Todo:
    // MONITORENTER,
    // MONITOREXIT,

    // Todo:
    // WIDE,

    // Todo:
    // MULTIANEWARRAY,

    IFNULL {
        offset: i32
    },
    IFNONNULL {
        offset: i32
    },
    GOTO_W {
        offset: i32
    },

    // Deprecated:
    // JSR_W,

    // Reserved:
    // BREAKPOINT,
    // IMPDEP1,
    // IMPDEP2,
}

impl Instruction {
    pub fn size(&self) -> usize {
        match self {
            Instruction::NOP => 1,
            Instruction::ACONST_NULL => 1,
            Instruction::ICONST_M1 => 1,
            Instruction::ICONST_0 => 1,
            Instruction::ICONST_1 => 1,
            Instruction::ICONST_2 => 1,
            Instruction::ICONST_3 => 1,
            Instruction::ICONST_4 => 1,
            Instruction::ICONST_5 => 1,
            Instruction::LCONST_0 => 1,
            Instruction::LCONST_1 => 1,
            Instruction::FCONST_0 => 1,
            Instruction::FCONST_1 => 1,
            Instruction::FCONST_2 => 1,
            Instruction::DCONST_0 => 1,
            Instruction::DCONST_1 => 1,
            Instruction::BIPUSH { .. } => 2,
            Instruction::SIPUSH { .. } => 3,
            Instruction::LDC { .. } => 2,
            Instruction::LDC_W { .. } => 3,
            Instruction::LDC2_W { .. } => 3,
            Instruction::ILOAD { .. } => 2,
            Instruction::LLOAD { .. } => 2,
            Instruction::FLOAD { .. } => 2,
            Instruction::DLOAD { .. } => 2,
            Instruction::ALOAD { .. } => 2,
            Instruction::ILOAD_0 => 1,
            Instruction::ILOAD_1 => 1,
            Instruction::ILOAD_2 => 1,
            Instruction::ILOAD_3 => 1,
            Instruction::LLOAD_0 => 1,
            Instruction::LLOAD_1 => 1,
            Instruction::LLOAD_2 => 1,
            Instruction::LLOAD_3 => 1,
            Instruction::FLOAD_0 => 1,
            Instruction::FLOAD_1 => 1,
            Instruction::FLOAD_2 => 1,
            Instruction::FLOAD_3 => 1,
            Instruction::DLOAD_0 => 1,
            Instruction::DLOAD_1 => 1,
            Instruction::DLOAD_2 => 1,
            Instruction::DLOAD_3 => 1,
            Instruction::ALOAD_0 => 1,
            Instruction::ALOAD_1 => 1,
            Instruction::ALOAD_2 => 1,
            Instruction::ALOAD_3 => 1,
            Instruction::IALOAD => 1,
            Instruction::LALOAD => 1,
            Instruction::FALOAD => 1,
            Instruction::DALOAD => 1,
            Instruction::AALOAD => 1,
            Instruction::BALOAD => 1,
            Instruction::CALOAD => 1,
            Instruction::SALOAD => 1,
            Instruction::ISTORE { .. } => 2,
            Instruction::LSTORE { .. } => 2,
            Instruction::FSTORE { .. } => 2,
            Instruction::DSTORE { .. } => 2,
            Instruction::ASTORE { .. } => 2,
            Instruction::ISTORE_0 => 1,
            Instruction::ISTORE_1 => 1,
            Instruction::ISTORE_2 => 1,
            Instruction::ISTORE_3 => 1,
            Instruction::LSTORE_0 => 1,
            Instruction::LSTORE_1 => 1,
            Instruction::LSTORE_2 => 1,
            Instruction::LSTORE_3 => 1,
            Instruction::FSTORE_0 => 1,
            Instruction::FSTORE_1 => 1,
            Instruction::FSTORE_2 => 1,
            Instruction::FSTORE_3 => 1,
            Instruction::DSTORE_0 => 1,
            Instruction::DSTORE_1 => 1,
            Instruction::DSTORE_2 => 1,
            Instruction::DSTORE_3 => 1,
            Instruction::ASTORE_0 => 1,
            Instruction::ASTORE_1 => 1,
            Instruction::ASTORE_2 => 1,
            Instruction::ASTORE_3 => 1,
            Instruction::IASTORE => 1,
            Instruction::LASTORE => 1,
            Instruction::FASTORE => 1,
            Instruction::DASTORE => 1,
            Instruction::AASTORE => 1,
            Instruction::BASTORE => 1,
            Instruction::CASTORE => 1,
            Instruction::SASTORE => 1,
            Instruction::POP => 1,
            Instruction::POP2 => 1,
            Instruction::DUP => 1,
            Instruction::DUP_X1 => 1,
            Instruction::DUP_X2 => 1,
            Instruction::DUP2 => 1,
            Instruction::DUP2_X1 => 1,
            Instruction::DUP2_X2 => 1,
            Instruction::SWAP => 1,
            Instruction::IADD => 1,
            Instruction::LADD => 1,
            Instruction::FADD => 1,
            Instruction::DADD => 1,
            Instruction::ISUB => 1,
            Instruction::LSUB => 1,
            Instruction::FSUB => 1,
            Instruction::DSUB => 1,
            Instruction::IMUL => 1,
            Instruction::LMUL => 1,
            Instruction::FMUL => 1,
            Instruction::DMUL => 1,
            Instruction::IDIV => 1,
            Instruction::LDIV => 1,
            Instruction::FDIV => 1,
            Instruction::DDIV => 1,
            Instruction::IREM => 1,
            Instruction::LREM => 1,
            Instruction::FREM => 1,
            Instruction::DREM => 1,
            Instruction::INEG => 1,
            Instruction::LNEG => 1,
            Instruction::FNEG => 1,
            Instruction::DNEG => 1,
            Instruction::ISHL => 1,
            Instruction::LSHL => 1,
            Instruction::ISHR => 1,
            Instruction::LSHR => 1,
            Instruction::IUSHR => 1,
            Instruction::LUSHR => 1,
            Instruction::IAND => 1,
            Instruction::LAND => 1,
            Instruction::IOR => 1,
            Instruction::LOR => 1,
            Instruction::IXOR => 1,
            Instruction::LXOR => 1,
            Instruction::IINC { .. } => 3,
            Instruction::I2L => 1,
            Instruction::I2F => 1,
            Instruction::I2D => 1,
            Instruction::L2I => 1,
            Instruction::L2F => 1,
            Instruction::L2D => 1,
            Instruction::F2I => 1,
            Instruction::F2L => 1,
            Instruction::F2D => 1,
            Instruction::D2I => 1,
            Instruction::D2L => 1,
            Instruction::D2F => 1,
            Instruction::I2B => 1,
            Instruction::I2C => 1,
            Instruction::I2S => 1,
            Instruction::LCMP => 1,
            Instruction::FCMPL => 1,
            Instruction::FCMPG => 1,
            Instruction::DCMPL => 1,
            Instruction::DCMPG => 1,
            Instruction::IFEQ { .. } => 3,
            Instruction::IFNE { .. } => 3,
            Instruction::IFLT { .. } => 3,
            Instruction::IFGE { .. } => 3,
            Instruction::IFGT { .. } => 3,
            Instruction::IFLE { .. } => 3,
            Instruction::IF_ICMPEQ { .. } => 3,
            Instruction::IF_ICMPNE { .. } => 3,
            Instruction::IF_ICMPLT { .. } => 3,
            Instruction::IF_ICMPGE { .. } => 3,
            Instruction::IF_ICMPGT { .. } => 3,
            Instruction::IF_ICMPLE { .. } => 3,
            Instruction::IF_ACMPEQ { .. } => 3,
            Instruction::IF_ACMPNE { .. } => 3,
            Instruction::GOTO { .. } => 3,
            Instruction::IRETURN => 1,
            Instruction::LRETURN => 1,
            Instruction::FRETURN => 1,
            Instruction::DRETURN => 1,
            Instruction::ARETURN => 1,
            Instruction::RETURN => 1,
            Instruction::GETSTATIC { .. } => 3,
            Instruction::PUTSTATIC { .. } => 3,
            Instruction::GETFIELD { .. } => 3,
            Instruction::PUTFIELD { .. } => 3,
            Instruction::INVOKEVIRTUAL { .. } => 3,
            Instruction::INVOKESPECIAL { .. } => 3,
            Instruction::INVOKESTATIC { .. } => 3,
            Instruction::NEW { .. } => 3,
            Instruction::NEWARRAY { .. } => 2,
            Instruction::ANEWARRAY { .. } => 3,
            Instruction::ARRAYLENGTH => 1,
            Instruction::INSTANCEOF { .. } => 3,
            Instruction::IFNULL { .. } => 3,
            Instruction::IFNONNULL { .. } => 3,
            Instruction::GOTO_W { .. } => 3,
            _ => unimplemented!()
        }
    }
}
