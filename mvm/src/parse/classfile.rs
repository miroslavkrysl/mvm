use crate::class::descriptor::{ReturnDescriptor, TypeDescriptor, ParamsDescriptor};
use crate::class::name::{ClassName, FieldName, MethodName};
use crate::instruction::Instruction;
use crate::class::class::Class;
use crate::parse::error::CreateClassError;
use std::convert::TryInto;
use crate::class::method::Method;
use crate::class::code::Code;
use crate::class::field::Field;
use std::sync::Arc;


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
    descriptor: TypeDescriptor,
    is_static: bool,
}


impl FieldInfo {
    pub fn new(name: FieldName, descriptor: TypeDescriptor, is_static: bool) -> Self {
        FieldInfo { name, descriptor, is_static }
    }
}


#[derive(Debug, Clone)]
pub struct MethodInfo {
    name: MethodName,
    return_desc: ReturnDescriptor,
    params_desc: ParamsDescriptor,
    is_static: bool,
    locals: u8,
    instructions: Vec<Instruction>,
}


impl MethodInfo {
    pub fn new(
        name: MethodName,
        return_desc: ReturnDescriptor,
        params_desc: ParamsDescriptor,
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
            fields.push(Arc::new(field_info.try_into()?));
        }

        let mut methods = Vec::new();
        for method_info in self.methods {
            methods.push(Arc::new(method_info.try_into()?));
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
            self.name,
            self.return_desc,
            self.params_desc,
            self.is_static,
            Code::new(self.locals as usize, self.instructions)?,
        )?)
    }
}

impl TryInto<Field> for FieldInfo {
    type Error = CreateClassError;

    fn try_into(self) -> Result<Field, Self::Error> {
        Ok(Field::new(
            self.descriptor,
            self.name,
            self.is_static
        ))
    }
}

