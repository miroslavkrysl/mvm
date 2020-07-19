use crate::vm::bytecode::instruction::Instruction;
use crate::vm::exec::thread::Thread;
use crate::vm::exec::error::ExecError;
use crate::vm::class::descriptor::{ReturnDesc, TypeDesc};
use crate::vm::types::value::ValueType;
use crate::vm::types::int::Int;
use crate::vm::types::long::Long;
use crate::vm::types::float::Float;
use crate::vm::types::double::Double;
use crate::vm::types::reference::Reference;

impl Instruction {
    pub(super) fn ireturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if *expected_type != ReturnDesc::NonVoid(TypeDesc::Int) {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Int
            })
        }

        let value = frame.stack().pop::<Int>()?;
        thread.stack().pop().expect("frame stack should not be empty");

        let frame = thread.stack().current().unwrap();
        frame.stack().push(value)?;
        Ok(())
    }

    pub(super) fn lreturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if *expected_type != ReturnDesc::NonVoid(TypeDesc::Long) {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Long
            })
        }

        let value = frame.stack().pop::<Long>()?;
        thread.stack().pop().expect("frame stack should not be empty");

        let frame = thread.stack().current().unwrap();
        frame.stack().push(value)?;
        Ok(())
    }

    pub(super) fn freturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if *expected_type != ReturnDesc::NonVoid(TypeDesc::Float) {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Float
            })
        }

        let value = frame.stack().pop::<Float>()?;
        thread.stack().pop().expect("frame stack should not be empty");

        let frame = thread.stack().current().unwrap();
        frame.stack().push(value)?;
        Ok(())
    }

    pub(super) fn dreturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if *expected_type != ReturnDesc::NonVoid(TypeDesc::Double) {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Double
            })
        }

        let value = frame.stack().pop::<Double>()?;
        thread.stack().pop().expect("frame stack should not be empty");
        let frame = thread.stack().current().unwrap();
        frame.stack().push(value)?;
        Ok(())
    }

    pub(super) fn areturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if let ReturnDesc::NonVoid(TypeDesc::Reference(class_name)) = expected_type {
            let value = frame.stack().pop::<Reference>()?;

            if !value.is_null() {
                let instance = value.as_instance().unwrap();

                if *instance.class().name() != *class_name {
                    return Err(ExecError::InvalidReturnReference {
                        expected: class_name.clone(),
                        found: instance.class().name().clone()
                    })
                }

            }

            thread.stack().pop().expect("frame stack should not be empty");
            let frame = thread.stack().current().unwrap();
            frame.stack().push(value)?;
            Ok(())
        } else {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Reference
            })
        }
    }

    pub(super) fn vreturn(&self, thread: &Thread) -> Result<(), ExecError> {
        let frame = thread.stack().current().unwrap();
        let expected_type = frame.method().signature().return_desc();

        if *expected_type != ReturnDesc::Void {
            return Err(ExecError::InvalidReturnType {
                expected: expected_type.clone(),
                called: ValueType::Double
            })
        }

        thread.stack().pop().expect("frame stack should not be empty");
        Ok(())
    }
}