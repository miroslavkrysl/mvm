use std::cell::{Cell, RefCell};
use std::fmt::Display;
use std::fmt;
use std::num::ParseIntError;
use std::str::{Lines, SplitWhitespace};

use thiserror::Error;
use crate::class::error::{DescriptorError, NameError};
use crate::classloader::classfile::{ClassInfo, FieldInfo, MethodInfo};
use crate::class::descriptor::{TypeDescriptor, ArrayDim, SimpleDescriptor, ReturnDescriptor, ArrayDescriptor};
use crate::instruction::Instruction;
use crate::class::name::{ClassName, MethodName, FieldName};
use regex::internal::Inst;
use crate::class::symbolic::{FieldSymRef, MethodSymRef};


pub struct Parser<'a> {
    input: Input<'a>
}


#[derive(Error, Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    line_pos: usize,
}


impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "on line {}: {}", self.line_pos, self.kind)
    }
}


#[derive(Error, Debug)]
pub enum ParseErrorKind {
    #[error("unknown entry: {0}")]
    UnknownEntry(String),
    #[error("unknown instruction: {0}")]
    UnknownInstruction(String),
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,
    #[error("unexpected end of line")]
    UnexpectedEndOfLine,
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("invalid method params descriptor: {0}")]
    InvalidParamsDescriptor(String),
    #[error("invalid field definition: {0}")]
    InvalidFieldDefinition(String),
    #[error("invalid method definition: {0}")]
    InvalidInstructionDefinition(String),
    #[error("invalid instruction definition: {0}")]
    InvalidMethodDefinition(String),
    #[error("invalid array descriptor: {0}")]
    InvalidArrayDescriptor(String),
    #[error("type descriptor is empty")]
    EmptyTypeDescriptor,
    #[error(transparent)]
    Desriptor {
        #[from]
        source: DescriptorError
    },
    #[error(transparent)]
    Name {
        #[from]
        source: NameError
    },
    #[error(transparent)]
    Int {
        #[from]
        source: ParseIntError
    },
}


impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser {
            input: Input::new(input)
        }
    }

    /// Parse the whole class file.
    pub fn parse(&self) -> Result<ClassInfo, ParseError> {
        self.parse_inner().map_err(|e| {
            ParseError {
                kind: e,
                line_pos: self.input.line_pos(),
            }
        })
    }
}


impl<'a> Parser<'a> {
    /// Parse the whole class file.
    fn parse_inner(&self) -> Result<ClassInfo, ParseErrorKind> {
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
                _ => return Err(ParseErrorKind::UnknownEntry(line.into())),
            }
        }

        Ok(ClassInfo::new(
            class_name,
            fields,
            methods,
        ))
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
    fn next_line_or_err(&self) -> Result<&str, ParseErrorKind> {
        self.next_line().ok_or_else(|| {
            ParseErrorKind::UnexpectedEndOfInput
        })
    }
}


impl<'a> Parser<'a> {
    /// Parse the whole class file.
    fn parse_field(&self) -> Result<FieldInfo, ParseErrorKind> {
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
            return Err(ParseErrorKind::InvalidFieldDefinition(line.into()));
        }

        let desc = self.parse_type_desc(desc.unwrap())?;
        let name = self.parse_field_name(name.unwrap())?;

        Ok(FieldInfo::new(name, desc, is_static))
    }


    /// Parse the whole class file.
    fn parse_method(&self) -> Result<MethodInfo, ParseErrorKind> {
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
            return Err(ParseErrorKind::InvalidMethodDefinition(line.into()));
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
            instructions
        ))
    }

    /// Parse the whole class file.
    fn parse_instructions(&self) -> Result<Vec<Instruction>, ParseErrorKind> {
        let mut instructions = Vec::new();

        while let Some(line) = self.next_line() {
            if line.is_empty() {
                continue;
            }

            let mut tokens = Tokens::whitespaces(line);

            let instruction = match tokens.next_or_err()? {
                "END" => break,
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
                    "IALOAD" => Instruction::IALOAD,
                    "LALOAD" => Instruction::LALOAD,
                    "FALOAD" => Instruction::FALOAD,
                    "DALOAD" => Instruction::DALOAD,
                    "AALOAD" => Instruction::AALOAD,
                    "BALOAD" => Instruction::BALOAD,
                    "SALOAD" => Instruction::SALOAD,
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
                    "IASTORE" => Instruction::IASTORE,
                    "LASTORE" => Instruction::LASTORE,
                    "FASTORE" => Instruction::FASTORE,
                    "DASTORE" => Instruction::DASTORE,
                    "AASTORE" => Instruction::AASTORE,
                    "BASTORE" => Instruction::BASTORE,
                    "SASTORE" => Instruction::SASTORE,
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
                        self.parse_i8(tokens.next_or_err()?)?,
                        self.parse_i8(tokens.next_or_err()?)?
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
                    "I2B" => Instruction::I2B,
                    "I2S" => Instruction::I2S,
                    "LCMP" => Instruction::LCMP,
                    "FCMPL" => Instruction::FCMPL,
                    "FCMPG" => Instruction::FCMPG,
                    "DCMPL" => Instruction::DCMPL,
                    "DCMPG" => Instruction::DCMPG,
                    "IFEQ" => Instruction::IFEQ(self.parse_i16(tokens.next_or_err()?)?),
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
                    "GETSTATIC" => Instruction::GETSTATIC(
                        FieldSymRef::new(
                            self.parse_type_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_field_name(tokens.next_or_err()?)?,
                        )
                    ),
                    "PUTSTATIC" => Instruction::PUTSTATIC(
                        FieldSymRef::new(
                            self.parse_type_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_field_name(tokens.next_or_err()?)?,
                        )
                    ),
                    "GETFIELD" => Instruction::GETFIELD(
                        FieldSymRef::new(
                            self.parse_type_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_field_name(tokens.next_or_err()?)?,
                        )
                    ),
                    "PUTFIELD" => Instruction::PUTFIELD(
                        FieldSymRef::new(
                            self.parse_type_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_field_name(tokens.next_or_err()?)?,
                        )
                    ),
                    "INVOKEVIRTUAL" => Instruction::INVOKEVIRTUAL(
                        MethodSymRef::new(
                            self.parse_return_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_method_name(tokens.next_or_err()?)?,
                            self.parse_method_params(tokens.next_or_err()?)?,
                        )
                    ),
                    "INVOKESPECIAL" => Instruction::INVOKESPECIAL(
                        MethodSymRef::new(
                            self.parse_return_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_method_name(tokens.next_or_err()?)?,
                            self.parse_method_params(tokens.next_or_err()?)?,
                        )
                    ),
                    "INVOKESTATIC" => Instruction::INVOKESTATIC(
                        MethodSymRef::new(
                            self.parse_return_desc(tokens.next_or_err()?)?,
                            self.parse_class_name(tokens.next_or_err()?)?,
                            self.parse_method_name(tokens.next_or_err()?)?,
                            self.parse_method_params(tokens.next_or_err()?)?,
                        )
                    ),
                    "ARRAYLENGTH" => Instruction::ARRAYLENGTH,
                    "NEW" => Instruction::NEW(self.parse_class_name(tokens.next_or_err()?)?),
                    "NEWARRAY" => Instruction::NEWARRAY(self.parse_type_desc(tokens.next_or_err()?)?),
                    _ => return Err(ParseErrorKind::UnknownInstruction(name.into()))
                },
            };

            if tokens.next().is_some() {
                return Err(ParseErrorKind::InvalidInstructionDefinition(line.into()));
            }

            instructions.push(instruction);
        }

        Ok(instructions)
    }


    /// Parse the class name.
    fn parse_class_name(&self, name: &str) -> Result<ClassName, ParseErrorKind> {
        let name = ClassName::new(name)?;
        Ok(name)
    }

    /// Parse the method name.
    fn parse_method_name(&self, name: &str) -> Result<MethodName, ParseErrorKind> {
        let name = MethodName::new(name)?;
        Ok(name)
    }

    /// Parse the field name.
    fn parse_field_name(&self, name: &str) -> Result<FieldName, ParseErrorKind> {
        let name = FieldName::new(name)?;
        Ok(name)
    }

    /// Parse type descriptor.
    fn parse_type_desc(&self, desc: &str) -> Result<TypeDescriptor, ParseErrorKind> {
        if desc.is_empty() {
            return Err(ParseErrorKind::EmptyTypeDescriptor);
        }

        if desc.starts_with("[") && desc.ends_with("]") {
            // array desc
            let mut tokens = desc[1..(desc.len() - 1)].split(";");
            let t = tokens.next();
            let d = tokens.next();

            if t.is_none() || d.is_none() || tokens.next().is_some() {
                // too few or too many items in array descriptor
                return Err(ParseErrorKind::InvalidArrayDescriptor(desc.into()));
            }

            let t = self.parse_simple_desc(t.unwrap())?;
            let d = self.parse_u8(d.unwrap())?;
            let dim = ArrayDim::new(d)?;

            return Ok(ArrayDescriptor::new(t, dim).into());
        }

        self.parse_simple_desc(desc).map(|d| d.into())
    }

    /// Parse type descriptor.
    fn parse_simple_desc(&self, desc: &str) -> Result<SimpleDescriptor, ParseErrorKind> {
        if desc.is_empty() {
            return Err(ParseErrorKind::EmptyTypeDescriptor);
        }

        Ok(match desc {
            "byte" => SimpleDescriptor::Byte,
            "short" => SimpleDescriptor::Short,
            "int" => SimpleDescriptor::Int,
            "long" => SimpleDescriptor::Long,
            "float" => SimpleDescriptor::Float,
            "double" => SimpleDescriptor::Double,
            class_name => SimpleDescriptor::Reference(self.parse_class_name(class_name)?),
        })
    }

    /// Parse return descriptor.
    fn parse_return_desc(&self, desc: &str) -> Result<ReturnDescriptor, ParseErrorKind> {
        if desc.is_empty() {
            return Err(ParseErrorKind::EmptyTypeDescriptor);
        }

        Ok(match desc {
            "void" => ReturnDescriptor::Void,
            t => self.parse_type_desc(t)?.into(),
        })
    }

    /// Parse type descriptor.
    fn parse_method_params(&self, desc: &str) -> Result<Vec<TypeDescriptor>, ParseErrorKind> {
        if !desc.starts_with("(") || !desc.ends_with(")") {
            return Err(ParseErrorKind::InvalidParamsDescriptor(desc.into()));
        }

        // array desc
        let desc = &desc[1..(desc.len() - 1)];

        if desc.is_empty() {
            return Ok(Vec::new());
        }

        let tokens = desc[1..(desc.len() - 1)].split(",");

        tokens.map(|t| self.parse_type_desc(t)).collect()
    }

    fn parse_u8(&self, i: &str) -> Result<u8, ParseErrorKind> {
        let n = i.parse::<u8>()?;
        Ok(n)
    }

    fn parse_i8(&self, i: &str) -> Result<i8, ParseErrorKind> {
        let n = i.parse::<i8>()?;
        Ok(n)
    }

    fn parse_i16(&self, i: &str) -> Result<i16, ParseErrorKind> {
        let n = i.parse::<i16>()?;
        Ok(n)
    }
}



struct Input<'a> {
    lines: RefCell<Lines<'a>>,
    line_pos: Cell<usize>,
}


impl<'a> Input<'a> {
    fn new(input: &'a str) -> Self {
        Input {
            lines: RefCell::new(input.lines()),
            line_pos: Cell::new(0),
        }
    }

    /// Get next line from input and update current line.
    /// Returns `None`, if there are no more lines.
    fn next_line(&self) -> Option<&str> {
        let line = self.lines.borrow_mut().next();

        if let Some(_) = line {
            self.line_pos.set(self.line_pos.get());
        }

        line
    }

    fn line_pos(&self) -> usize {
        self.line_pos.get()
    }
}



struct Tokens<'a, I>
    where I: Iterator<Item=&'a str>
{
    tokens: I,
}

impl<'a, I> Tokens<'a, I>
    where I: Iterator<Item=&'a str>
{

    fn next(&mut self) -> Option<&'a str> {
        self.tokens.next()
    }

    fn next_or_err(&mut self) -> Result<&'a str, ParseErrorKind> {
        self.next()
            .ok_or(ParseErrorKind::UnexpectedEndOfLine)
    }
}

impl<'a> Tokens<'a, SplitWhitespace<'a>> {
    fn whitespaces(input: &'a str) -> Self {
        Tokens {
            tokens: input.split_whitespace()
        }
    }
}