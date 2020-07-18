use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::error::ExecError;
use crate::vm::class::symbolic::{FieldRef, MethodRef};
use crate::vm::class::name::ClassName;


impl Instruction {
    fn getstatic(&self, thread: &Thread, field_ref: FieldRef) -> Result<(), ExecError> {
        // let class = thread.runtime().resolve_class(field_ref.class_name())?;
        // let value = class.static_field_value(field_ref.signature())?;
        // frame.stack().push_value(value)?;
        // frame.inc_pc();
        unimplemented!()
    }

    fn putstatic(&self, thread: &Thread, field_ref: FieldRef) -> Result<(), ExecError> {
        unimplemented!()
    }

    fn getfield(&self, thread: &Thread, field_ref: FieldRef) -> Result<(), ExecError> {
        unimplemented!()
    }

    fn putfield(&self, thread: &Thread, field_ref: FieldRef) -> Result<(), ExecError> {
        unimplemented!()
    }

    fn invokevirtual(&self, thread: &Thread, method_ref: MethodRef) -> Result<(), ExecError> {
        unimplemented!()
    }

    fn invokestatic(&self, thread: &Thread, method_ref: MethodRef) -> Result<(), ExecError> {
        unimplemented!()
    }

    fn new(&self, thread: &Thread, class_name: ClassName) -> Result<(), ExecError> {
        unimplemented!()
    }
}