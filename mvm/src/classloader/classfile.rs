use crate::class::descriptor::{ReturnDescriptor, TypeDescriptor};
use crate::class::name::{ClassName, FieldName, MethodName};
use crate::instruction::Instruction;


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
    params_desc: Vec<TypeDescriptor>,
    is_static: bool,
    locals: u8,
    instructions: Vec<Instruction>,
}


impl MethodInfo {
    pub fn new(
        name: MethodName,
        return_desc: ReturnDescriptor,
        params_desc: Vec<TypeDescriptor>,
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




