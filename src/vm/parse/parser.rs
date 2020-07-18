use std::cell::{Cell, RefCell};
use std::str::{Lines, SplitWhitespace};

use crate::vm::class::descriptor::{ParamsDesc, ReturnDesc, TypeDesc};
use crate::vm::class::name::{ClassName, FieldName, MethodName};
use crate::vm::class::signature::{FieldSig, MethodSig};
use crate::vm::class::symbolic::{FieldRef, MethodRef};
use crate::vm::parse::classfile::{ClassInfo, FieldInfo, MethodInfo};
use crate::vm::parse::error::{ParseClassError, ParseClassErrorKind, ParseNumberError};
use crate::vm::bytecode::instruction::{Instruction, Ldc2Arg, LdcArg};


/// MVM class file parser.
pub struct ClassFileParser<'a> {
    input: Input<'a>,
}


impl<'a> ClassFileParser<'a> {
    /// Create a classfile parser from given str input.
    pub fn new(input: &str) -> ClassFileParser {
        ClassFileParser {
            input: Input::new(input),
        }
    }

    /// Parse the whole class file.
    pub fn parse(self) -> Result<ClassInfo, ParseClassError> {
        self.parse_class_file()
            .map_err(|e| ParseClassError::new(e, self.input.line_pos()))
    }
}


impl<'a> ClassFileParser<'a> {
    /// Parse the whole class file.
    fn parse_class_file(&self) -> Result<ClassInfo, ParseClassErrorKind> {
        // class name
        let class_name = self.parse_class_name(self.next_line_or_err()?)?;

        // parse fields and methods
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while let Some(line) = self.next_line() {
            match line.trim() {
                "FIELD" => {
                    let info = self.parse_field()?;
                    fields.push(info);
                }
                "METHOD" => {
                    let info = self.parse_method()?;
                    methods.push(info);
                }
                _ => return Err(ParseClassErrorKind::UnknownEntry(line.into())),
            }
        }

        Ok(ClassInfo::new(class_name, fields, methods))
    }

    /// Get next non empty line from input. Comments are stripped
    /// and line is trimmed from whitespaces.
    fn next_line(&self) -> Option<&str> {
        while let Some(mut line) = self.input.next_line() {
            // strip comment
            line = match line.find("//") {
                None => line,
                Some(i) => line.split_at(i).0,
            };

            line = line.trim();
            if line.is_empty() {
                continue;
            }

            return Some(line);
        }

        None
    }

    /// Get next trimmed non-empty line stripped from comments.
    ///
    /// # Errors
    ///
    /// When no line is available, returns `ParseError::UnexpectedEndOfInput`
    fn next_line_or_err(&self) -> Result<&str, ParseClassErrorKind> {
        self.next_line()
            .ok_or_else(|| ParseClassErrorKind::UnexpectedEndOfInput)
    }
}


impl<'a> ClassFileParser<'a> {
    fn parse_field(&self) -> Result<FieldInfo, ParseClassErrorKind> {
        let line = self.next_line_or_err()?;
        let mut tokens = line.split_whitespace();

        // check static flag
        let mut is_static = false;
        let mut token = tokens.next();

        if token.is_some() && token.unwrap() == "static" {
            is_static = true;
            token = tokens.next()
        }

        let desc = token;
        let name = tokens.next();

        if desc.is_none() || name.is_none() || tokens.next().is_some() {
            // too few or too many items in field definition
            return Err(ParseClassErrorKind::InvalidFieldDefinition(line.into()));
        }

        let desc = self.parse_type_desc(desc.unwrap())?;
        let name = self.parse_field_name(name.unwrap())?;

        Ok(FieldInfo::new(name, desc, is_static))
    }

    fn parse_method(&self) -> Result<MethodInfo, ParseClassErrorKind> {
        let line = self.next_line_or_err()?;
        let mut tokens = line.split_whitespace();

        // check static flag
        let mut is_static = false;
        let mut token = tokens.next();

        if token.is_some() && token.unwrap() == "static" {
            is_static = true;
            token = tokens.next()
        }

        let ret = token;
        let name = tokens.next();
        let params = tokens.next();
        let locals = tokens.next();

        if ret.is_none() || name.is_none() || params.is_none() || tokens.next().is_some() {
            // too few or too many items in method definition
            return Err(ParseClassErrorKind::InvalidMethodDefinition(line.into()));
        }

        let ret = self.parse_return_desc(ret.unwrap())?;
        let name = self.parse_method_name(name.unwrap())?;
        let params = self.parse_method_params(params.unwrap())?;
        let locals = self.parse_u8(locals.unwrap())?;

        let instructions = self.parse_instructions()?;

        Ok(MethodInfo::new(
            name,
            ret,
            params,
            is_static,
            locals,
            instructions,
        ))
    }

    /// Parse all instructions line by line until the line with
    /// keyword `END` is reached.
    fn parse_instructions(&self) -> Result<Vec<Instruction>, ParseClassErrorKind> {
        let mut instructions = Vec::new();
        let mut ended = false;

        while let Some(line) = self.next_line() {
            if line.is_empty() {
                continue;
            }

            let mut tokens = Tokens::whitespaces(line);

            let instruction = match tokens.next_or_err()? {
                "END" => {
                    ended = true;
                    break;
                }
                name => match name {
                    "NOP" => Instruction::NOP,
                    "ACONST_NULL" => Instruction::ACONST_NULL,
                    "ICONST_M1" => Instruction::ICONST_M1,
                    "ICONST_0" => Instruction::ICONST_0,
                    "ICONST_1" => Instruction::ICONST_1,
                    "ICONST_2" => Instruction::ICONST_2,
                    "ICONST_3" => Instruction::ICONST_3,
                    "ICONST_4" => Instruction::ICONST_4,
                    "ICONST_5" => Instruction::ICONST_5,
                    "LCONST_0" => Instruction::LCONST_0,
                    "LCONST_1" => Instruction::LCONST_1,
                    "FCONST_0" => Instruction::FCONST_0,
                    "FCONST_1" => Instruction::FCONST_1,
                    "FCONST_2" => Instruction::FCONST_2,
                    "DCONST_0" => Instruction::DCONST_0,
                    "DCONST_1" => Instruction::DCONST_1,
                    "BIPUSH" => Instruction::BIPUSH(self.parse_i8(tokens.next_or_err()?)?),
                    "SIPUSH" => Instruction::SIPUSH(self.parse_i16(tokens.next_or_err()?)?),
                    "LDC" => Instruction::LDC(self.parse_ldc_arg(tokens.next_or_err()?)?),
                    "LDC_W" => Instruction::LDC_W(self.parse_ldc_arg(tokens.next_or_err()?)?),
                    "LDC2_W" => Instruction::LDC2_W(self.parse_ldc2_arg(tokens.next_or_err()?)?),
                    "ILOAD" => Instruction::ILOAD(self.parse_u8(tokens.next_or_err()?)?),
                    "LLOAD" => Instruction::LLOAD(self.parse_u8(tokens.next_or_err()?)?),
                    "FLOAD" => Instruction::FLOAD(self.parse_u8(tokens.next_or_err()?)?),
                    "DLOAD" => Instruction::DLOAD(self.parse_u8(tokens.next_or_err()?)?),
                    "ALOAD" => Instruction::ALOAD(self.parse_u8(tokens.next_or_err()?)?),
                    "ILOAD_0" => Instruction::ILOAD_0,
                    "ILOAD_1" => Instruction::ILOAD_1,
                    "ILOAD_2" => Instruction::ILOAD_2,
                    "ILOAD_3" => Instruction::ILOAD_3,
                    "LLOAD_0" => Instruction::LLOAD_0,
                    "LLOAD_1" => Instruction::LLOAD_1,
                    "LLOAD_2" => Instruction::LLOAD_2,
                    "LLOAD_3" => Instruction::LLOAD_3,
                    "FLOAD_0" => Instruction::FLOAD_0,
                    "FLOAD_1" => Instruction::FLOAD_1,
                    "FLOAD_2" => Instruction::FLOAD_2,
                    "FLOAD_3" => Instruction::FLOAD_3,
                    "DLOAD_0" => Instruction::DLOAD_0,
                    "DLOAD_1" => Instruction::DLOAD_1,
                    "DLOAD_2" => Instruction::DLOAD_2,
                    "DLOAD_3" => Instruction::DLOAD_3,
                    "ALOAD_0" => Instruction::ALOAD_0,
                    "ALOAD_1" => Instruction::ALOAD_1,
                    "ALOAD_2" => Instruction::ALOAD_2,
                    "ALOAD_3" => Instruction::ALOAD_3,
                    "ISTORE" => Instruction::ISTORE(self.parse_u8(tokens.next_or_err()?)?),
                    "LSTORE" => Instruction::LSTORE(self.parse_u8(tokens.next_or_err()?)?),
                    "FSTORE" => Instruction::FSTORE(self.parse_u8(tokens.next_or_err()?)?),
                    "DSTORE" => Instruction::DSTORE(self.parse_u8(tokens.next_or_err()?)?),
                    "ASTORE" => Instruction::ASTORE(self.parse_u8(tokens.next_or_err()?)?),
                    "ISTORE_0" => Instruction::ISTORE_0,
                    "ISTORE_1" => Instruction::ISTORE_1,
                    "ISTORE_2" => Instruction::ISTORE_2,
                    "ISTORE_3" => Instruction::ISTORE_3,
                    "LSTORE_0" => Instruction::LSTORE_0,
                    "LSTORE_1" => Instruction::LSTORE_1,
                    "LSTORE_2" => Instruction::LSTORE_2,
                    "LSTORE_3" => Instruction::LSTORE_3,
                    "FSTORE_0" => Instruction::FSTORE_0,
                    "FSTORE_1" => Instruction::FSTORE_1,
                    "FSTORE_2" => Instruction::FSTORE_2,
                    "FSTORE_3" => Instruction::FSTORE_3,
                    "DSTORE_0" => Instruction::DSTORE_0,
                    "DSTORE_1" => Instruction::DSTORE_1,
                    "DSTORE_2" => Instruction::DSTORE_2,
                    "DSTORE_3" => Instruction::DSTORE_3,
                    "ASTORE_0" => Instruction::ASTORE_0,
                    "ASTORE_1" => Instruction::ASTORE_1,
                    "ASTORE_2" => Instruction::ASTORE_2,
                    "ASTORE_3" => Instruction::ASTORE_3,
                    "POP" => Instruction::POP,
                    "POP2" => Instruction::POP2,
                    "DUP" => Instruction::DUP,
                    "DUP_X1" => Instruction::DUP_X1,
                    "DUP_X2" => Instruction::DUP_X2,
                    "DUP2" => Instruction::DUP2,
                    "DUP2_X1" => Instruction::DUP2_X1,
                    "DUP2_X2" => Instruction::DUP2_X2,
                    "SWAP" => Instruction::SWAP,
                    "IADD" => Instruction::IADD,
                    "LADD" => Instruction::LADD,
                    "FADD" => Instruction::FADD,
                    "DADD" => Instruction::DADD,
                    "ISUB" => Instruction::ISUB,
                    "LSUB" => Instruction::LSUB,
                    "FSUB" => Instruction::FSUB,
                    "DSUB" => Instruction::DSUB,
                    "IMUL" => Instruction::IMUL,
                    "LMUL" => Instruction::LMUL,
                    "FMUL" => Instruction::FMUL,
                    "DMUL" => Instruction::DMUL,
                    "IDIV" => Instruction::IDIV,
                    "LDIV" => Instruction::LDIV,
                    "FDIV" => Instruction::FDIV,
                    "DDIV" => Instruction::DDIV,
                    "IREM" => Instruction::IREM,
                    "LREM" => Instruction::LREM,
                    "FREM" => Instruction::FREM,
                    "DREM" => Instruction::DREM,
                    "INEG" => Instruction::INEG,
                    "LNEG" => Instruction::LNEG,
                    "FNEG" => Instruction::FNEG,
                    "DNEG" => Instruction::DNEG,
                    "ISHL" => Instruction::ISHL,
                    "LSHL" => Instruction::LSHL,
                    "ISHR" => Instruction::ISHR,
                    "LSHR" => Instruction::LSHR,
                    "IUSHR" => Instruction::IUSHR,
                    "LUSHR" => Instruction::LUSHR,
                    "IAND" => Instruction::IAND,
                    "LAND" => Instruction::LAND,
                    "IOR" => Instruction::IOR,
                    "LOR" => Instruction::LOR,
                    "IXOR" => Instruction::IXOR,
                    "LXOR" => Instruction::LXOR,
                    "IINC" => Instruction::IINC(
                        self.parse_u8(tokens.next_or_err()?)?,
                        self.parse_i8(tokens.next_or_err()?)?,
                    ),
                    "I2L" => Instruction::I2L,
                    "I2F" => Instruction::I2F,
                    "I2D" => Instruction::I2D,
                    "L2I" => Instruction::L2I,
                    "L2F" => Instruction::L2F,
                    "L2D" => Instruction::L2D,
                    "F2I" => Instruction::F2I,
                    "F2L" => Instruction::F2L,
                    "F2D" => Instruction::F2D,
                    "D2I" => Instruction::D2I,
                    "D2L" => Instruction::D2L,
                    "D2F" => Instruction::D2F,
                    "LCMP" => Instruction::LCMP,
                    "FCMPL" => Instruction::FCMPL,
                    "FCMPG" => Instruction::FCMPG,
                    "DCMPL" => Instruction::DCMPL,
                    "DCMPG" => Instruction::DCMPG,
                    "IFEQ" => Instruction::IFEQ(self.parse_i16(tokens.next_or_err()?)?),
                    "IFNE" => Instruction::IFNE(self.parse_i16(tokens.next_or_err()?)?),
                    "IFLT" => Instruction::IFLT(self.parse_i16(tokens.next_or_err()?)?),
                    "IFGE" => Instruction::IFGE(self.parse_i16(tokens.next_or_err()?)?),
                    "IFGT" => Instruction::IFGT(self.parse_i16(tokens.next_or_err()?)?),
                    "IFLE" => Instruction::IFLE(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPEQ" => Instruction::IF_ICMPEQ(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPNE" => Instruction::IF_ICMPNE(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPLT" => Instruction::IF_ICMPLT(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPGE" => Instruction::IF_ICMPGE(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPGT" => Instruction::IF_ICMPGT(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ICMPLE" => Instruction::IF_ICMPLE(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ACMPEQ" => Instruction::IF_ACMPEQ(self.parse_i16(tokens.next_or_err()?)?),
                    "IF_ACMPNE" => Instruction::IF_ACMPNE(self.parse_i16(tokens.next_or_err()?)?),
                    "GOTO" => Instruction::GOTO(self.parse_i16(tokens.next_or_err()?)?),
                    "IFNULL" => Instruction::IFNULL(self.parse_i16(tokens.next_or_err()?)?),
                    "IFNONNULL" => Instruction::IFNONNULL(self.parse_i16(tokens.next_or_err()?)?),
                    "IRETURN" => Instruction::IRETURN,
                    "LRETURN" => Instruction::LRETURN,
                    "FRETURN" => Instruction::FRETURN,
                    "DRETURN" => Instruction::DRETURN,
                    "ARETURN" => Instruction::ARETURN,
                    "RETURN" => Instruction::RETURN,
                    "GETSTATIC" => {
                        let t = self.parse_type_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_field_name(tokens.next_or_err()?)?;
                        Instruction::GETSTATIC(FieldRef::new(c, FieldSig::new(t, n)))
                    }
                    "PUTSTATIC" => {
                        let t = self.parse_type_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_field_name(tokens.next_or_err()?)?;
                        Instruction::PUTSTATIC(FieldRef::new(c, FieldSig::new(t, n)))
                    }
                    "GETFIELD" => {
                        let t = self.parse_type_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_field_name(tokens.next_or_err()?)?;
                        Instruction::GETFIELD(FieldRef::new(c, FieldSig::new(t, n)))
                    }
                    "PUTFIELD" => {
                        let t = self.parse_type_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_field_name(tokens.next_or_err()?)?;
                        Instruction::PUTFIELD(FieldRef::new(c, FieldSig::new(t, n)))
                    }
                    "INVOKEVIRTUAL" => {
                        let r = self.parse_return_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_method_name(tokens.next_or_err()?)?;
                        let p = self.parse_method_params(tokens.next_or_err()?)?;
                        Instruction::INVOKEVIRTUAL(MethodRef::new(c, MethodSig::new(r, n, p)?))
                    }
                    "INVOKESTATIC" => {
                        let r = self.parse_return_desc(tokens.next_or_err()?)?;
                        let c = self.parse_class_name(tokens.next_or_err()?)?;
                        let n = self.parse_method_name(tokens.next_or_err()?)?;
                        let p = self.parse_method_params(tokens.next_or_err()?)?;
                        Instruction::INVOKESTATIC(MethodRef::new(c, MethodSig::new(r, n, p)?))
                    }
                    "NEW" => Instruction::NEW(self.parse_class_name(tokens.next_or_err()?)?),
                    _ => return Err(ParseClassErrorKind::UnknownInstruction(name.into())),
                },
            };

            if tokens.next().is_some() {
                return Err(ParseClassErrorKind::InvalidInstructionDefinition(
                    line.into(),
                ));
            }

            instructions.push(instruction);
        }

        if !ended {
            self.next_line_or_err()?;
        }

        Ok(instructions)
    }

    fn parse_class_name(&self, name: &str) -> Result<ClassName, ParseClassErrorKind> {
        let name = ClassName::new(name)?;
        Ok(name)
    }

    fn parse_method_name(&self, name: &str) -> Result<MethodName, ParseClassErrorKind> {
        let name = MethodName::new(name)?;
        Ok(name)
    }

    fn parse_field_name(&self, name: &str) -> Result<FieldName, ParseClassErrorKind> {
        let name = FieldName::new(name)?;
        Ok(name)
    }

    fn parse_type_desc(&self, desc: &str) -> Result<TypeDesc, ParseClassErrorKind> {
        if desc.is_empty() {
            return Err(ParseClassErrorKind::EmptyTypeDescriptor);
        }

        Ok(match desc {
            "int" => TypeDesc::Int,
            "long" => TypeDesc::Long,
            "float" => TypeDesc::Float,
            "double" => TypeDesc::Double,
            class_name => TypeDesc::Reference(self.parse_class_name(class_name)?),
        })
    }

    fn parse_return_desc(&self, desc: &str) -> Result<ReturnDesc, ParseClassErrorKind> {
        if desc.is_empty() {
            return Err(ParseClassErrorKind::EmptyTypeDescriptor);
        }

        Ok(match desc {
            "void" => ReturnDesc::Void,
            t => self.parse_type_desc(t)?.into(),
        })
    }

    fn parse_method_params(&self, desc: &str) -> Result<ParamsDesc, ParseClassErrorKind> {
        if !desc.starts_with("(") || !desc.ends_with(")") {
            return Err(ParseClassErrorKind::InvalidParamsDescriptor(desc.into()));
        }

        let desc = &desc[1..(desc.len() - 1)];

        if desc.is_empty() {
            return Ok(ParamsDesc::empty());
        }

        let tokens = desc.split(",");

        tokens.map(|t| self.parse_type_desc(t)).collect()
    }

    fn parse_u8(&self, i: &str) -> Result<u8, ParseNumberError> {
        let n = i.parse::<u8>()?;
        Ok(n)
    }

    fn parse_i8(&self, i: &str) -> Result<i8, ParseNumberError> {
        let n = i.parse::<i8>()?;
        Ok(n)
    }

    fn parse_i16(&self, i: &str) -> Result<i16, ParseNumberError> {
        let n = i.parse::<i16>()?;
        Ok(n)
    }

    fn parse_ldc_arg(&self, i: &str) -> Result<LdcArg, ParseNumberError> {
        match i.parse::<i32>() {
            Ok(n) => Ok(LdcArg::Int(n)),
            Err(ei) => match i.parse::<f32>() {
                Ok(n) => Ok(LdcArg::Float(n)),
                Err(ef) => Err(ParseNumberError::NotIntOrFloat {
                    i_error: ei,
                    f_error: ef,
                }),
            },
        }
    }

    fn parse_ldc2_arg(&self, i: &str) -> Result<Ldc2Arg, ParseNumberError> {
        match i.parse::<i64>() {
            Ok(n) => Ok(Ldc2Arg::Long(n)),
            Err(ei) => match i.parse::<f64>() {
                Ok(n) => Ok(Ldc2Arg::Double(n)),
                Err(ef) => Err(ParseNumberError::NotLongOrDouble {
                    i_error: ei,
                    f_error: ef,
                }),
            },
        }
    }
}


/// Helper struct for input tokenized into lines.
struct Input<'a> {
    lines: RefCell<Lines<'a>>,
    line_pos: Cell<usize>,
}


impl<'a> Input<'a> {
    /// Create new input by tokenizing the input into lines.
    fn new(input: &'a str) -> Self {
        Input {
            lines: RefCell::new(input.lines()),
            line_pos: Cell::new(0),
        }
    }

    /// Get next line from input and update current line.
    /// Returns `None`, if there are no more lines.
    /// [None][None]
    fn next_line(&self) -> Option<&str> {
        let line = self.lines.borrow_mut().next();

        if let Some(_) = line {
            self.line_pos.set(self.line_pos.get() + 1);
        }

        line
    }

    fn line_pos(&self) -> usize {
        self.line_pos.get()
    }
}


/// Helper struct for tokenized input.
struct Tokens<'a, I>
    where
        I: Iterator<Item=&'a str>,
{
    tokens: I,
}


impl<'a, I> Tokens<'a, I>
    where
        I: Iterator<Item=&'a str>,
{
    /// Get next token.
    fn next(&mut self) -> Option<&'a str> {
        self.tokens.next()
    }

    /// Returns next available token.
    ///
    /// # Errors
    ///
    /// Returns `ParseClassErrorKind::UnexpectedEndOfLine` if there
    /// are no more available tokens.
    fn next_or_err(&mut self) -> Result<&'a str, ParseClassErrorKind> {
        self.next().ok_or(ParseClassErrorKind::UnexpectedEndOfLine)
    }
}


impl<'a> Tokens<'a, SplitWhitespace<'a>> {
    fn whitespaces(input: &'a str) -> Self {
        Tokens {
            tokens: input.split_whitespace(),
        }
    }
}
