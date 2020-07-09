use crate::class::symbolic::{MethodSymRef, FieldSymRef};
use crate::class::name::ClassName;
use crate::class::descriptor::{TypeDescriptor, ArrayDim, ArrayDescriptor};
use crate::types::int::Int;
use crate::types::float::Float;
use crate::types::long::Long;
use crate::types::double::Double;


#[derive(Debug, Clone, PartialEq)]
pub enum LdcArg {
    Int(Int),
    Float(Float),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ldc2Arg {
    Long(Long),
    Double(Double),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    NOP,

    // push constants
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

    // push constant args
    BIPUSH(i8),
    SIPUSH(i16),
    LDC(LdcArg),
    LDC_W(Ldc2Arg),
    LDC2_W(Ldc2Arg),

    // load from locals and push
    ILOAD(u8),
    LLOAD(u8),
    FLOAD(u8),
    DLOAD(u8),
    ALOAD(u8),
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

    // load from array and push
    IALOAD,
    LALOAD,
    FALOAD,
    DALOAD,
    AALOAD,
    BALOAD,
    SALOAD,

    // pop and store in locals
    ISTORE(u8),
    LSTORE(u8),
    FSTORE(u8),
    DSTORE(u8),
    ASTORE(u8),
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

    // pop and store in array
    IASTORE,
    LASTORE,
    FASTORE,
    DASTORE,
    AASTORE,
    BASTORE,
    SASTORE,

    // stack manipulation
    POP,
    POP2,
    DUP,
    DUP_X1,
    DUP_X2,
    DUP2,
    DUP2_X1,
    DUP2_X2,
    SWAP,

    // arithmetic
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
    IINC (i8, i8),

    // conversions
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
    I2S,

    // comparisons
    LCMP,
    FCMPL,
    FCMPG,
    DCMPL,
    DCMPG,
    IFEQ(i16),
    IFLT(i16),
    IFGE(i16),
    IFGT(i16),
    IFLE(i16),

    IF_ICMPEQ(i16),
    IF_ICMPNE(i16),
    IF_ICMPLT(i16),
    IF_ICMPGE(i16),
    IF_ICMPGT(i16),
    IF_ICMPLE(i16),
    IF_ACMPEQ(i16),
    IF_ACMPNE(i16),
    GOTO(i16),
    IFNULL(i16),
    IFNONNULL(i16),

    // returns
    IRETURN,
    LRETURN,
    FRETURN,
    DRETURN,
    ARETURN,
    RETURN,

    // object access
    GETSTATIC(FieldSymRef),
    PUTSTATIC(FieldSymRef),
    GETFIELD(FieldSymRef),
    PUTFIELD(FieldSymRef),
    INVOKEVIRTUAL(MethodSymRef),
    INVOKESPECIAL(MethodSymRef),
    INVOKESTATIC(MethodSymRef),
    ARRAYLENGTH,

    // object creation
    NEW(ClassName),
    NEWARRAY(TypeDescriptor),
}