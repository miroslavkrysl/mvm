use crate::class::descriptor::{ReturnDesc, TypeDesc, ParamsDesc};
use crate::class::name::{ClassName, FieldName, MethodName};
use crate::instruction::Instruction;
use crate::class::class::Class;
use crate::parse::error::CreateClassError;
use std::convert::TryInto;
use crate::class::method::Method;
use crate::class::code::Code;
use crate::class::field::Field;
use std::sync::Arc;
use crate::class::signature::{FieldSig, MethodSig};


#[derive(Debug, Clone)]
pub struct ClassInfo {
    name: ClassName,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
}


impl ClassInfo {
    pub fn new(name: ClassName, fields: Vec<FieldInfo>, methods: Vec<MethodInfo>) -> Self {
        ClassInfo { name, fields, methods }
    }
}


#[derive(Debug, Clone)]
pub struct FieldInfo {
    name: FieldName,
    type_dec: TypeDesc,
    is_static: bool,
}


impl FieldInfo {
    pub fn new(name: FieldName, descriptor: TypeDesc, is_static: bool) -> Self {
        FieldInfo { name, type_dec: descriptor, is_static }
    }
}


#[derive(Debug, Clone)]
pub struct MethodInfo {
    name: MethodName,
    return_desc: ReturnDesc,
    params_desc: ParamsDesc,
    is_static: bool,
    locals: u8,
    instructions: Vec<Instruction>,
}


impl MethodInfo {
    pub fn new(
        name: MethodName,
        return_desc: ReturnDesc,
        params_desc: ParamsDesc,
        is_static: bool,
        locals: u8,
        instructions: Vec<Instruction>,
    ) -> Self {
        MethodInfo {
            name,
            return_desc,
            params_desc,
            is_static,
            locals,
            instructions,
        }
    }
}


impl TryInto<Class> for ClassInfo {
    type Error = CreateClassError;

    fn try_into(self) -> Result<Class, Self::Error> {
        let mut fields = Vec::new();
        for field_info in self.fields {
            fields.push(field_info.try_into()?);
        }

        let mut methods = Vec::new();
        for method_info in self.methods {
            methods.push(method_info.try_into()?);
        }

        Ok(Class::new(
            self.name,
            fields,
            methods
        )?)
    }
}

impl TryInto<Method> for MethodInfo {
    type Error = CreateClassError;

    fn try_into(self) -> Result<Method, Self::Error> {
        Ok(Method::new(
            MethodSig::new(self.return_desc, self.name, self.params_desc)?,
            self.is_static,
            Code::new(self.locals as usize, self.instructions)?,
        )?)
    }
}

impl TryInto<Field> for FieldInfo {
    type Error = CreateClassError;

    fn try_into(self) -> Result<Field, Self::Error> {
        Ok(Field::new(
            FieldSig::new(self.type_dec, self.name),
            self.is_static
        ))
    }
}

