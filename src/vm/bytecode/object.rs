use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::error::ExecError;
use crate::vm::class::symbolic::{FieldRef, MethodRef};
use crate::vm::class::name::ClassName;
use crate::vm::types::reference::Reference;
use crate::vm::memory::frame::Frame;

impl Instruction {
    pub(super) fn getstatic(&self, thread: &Thread, field_ref: &FieldRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(field_ref.class_name())?;
        let value = class.static_field_value(field_ref.signature())?;
        frame.stack().push_value(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn putstatic(&self, thread: &Thread, field_ref: &FieldRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(field_ref.class_name())?;
        let value = frame.stack().pop_value()?;
        class.set_static_field_value(field_ref.signature(), value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn getfield(&self, thread: &Thread, field_ref: &FieldRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(field_ref.class_name())?;
        let instance = frame.stack().pop::<Reference>()?.to_instance()?;
        let value = class.instance_field_value(&instance, field_ref.signature())?;
        frame.stack().push_value(value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn putfield(&self, thread: &Thread, field_ref: &FieldRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(field_ref.class_name())?;
        let value = frame.stack().pop_value()?;
        let instance = frame.stack().pop::<Reference>()?.to_instance()?;
        class.set_instance_field_value(&instance, field_ref.signature(), value)?;
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn invokevirtual(&self, thread: &Thread, method_ref: &MethodRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(method_ref.class_name())?;
        let method = class.instance_method(method_ref.signature())?;
        let next_frame = Frame::new_from_call(class.clone(), method.clone(), frame.stack())?;
        thread.stack().push(next_frame);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn invokestatic(&self, thread: &Thread, method_ref: &MethodRef) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(method_ref.class_name())?;
        let method = class.static_method(method_ref.signature())?;
        let next_frame = Frame::new_from_call(class.clone(), method.clone(), frame.stack())?;
        thread.stack().push(next_frame);
        frame.inc_pc();
        Ok(())
    }

    pub(super) fn new(&self, thread: &Thread, class_name: &ClassName) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let class = thread.runtime().resolve_class(class_name)?;
        let instance = thread.runtime().create_instance(class);
        let reference = Reference::new(instance);
        frame.stack().push(reference)?;
        frame.inc_pc();
        Ok(())
    }
}