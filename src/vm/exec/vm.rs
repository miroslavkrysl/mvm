use crate::vm::class::signature::{FieldSig, MethodSig};
use crate::vm::types::value::Value;
use std::sync::Arc;
use crate::vm::class::class::Class;
use crate::vm::class::field::Field;
use crate::vm::class::name::{ClassName, MethodName};
use std::path::PathBuf;
use crate::vm::class::instance::{InstancePtr, InstanceId};
use crate::vm::class::method::Method;
use crate::vm::exec::error::{ExecError, RuntimeError};
use crate::vm::exec::runtime::Runtime;
use crate::vm::class::descriptor::{ReturnDesc, ParamsDesc};


pub struct VirtualMachine {
    runtime: Arc<Runtime>,
    main_class: ClassName,
}

impl VirtualMachine {
    pub fn new(main_class: ClassName, class_path: Vec<PathBuf>) -> Self {
        VirtualMachine {
            runtime: Arc::new(Runtime::new(class_path)),
            main_class
        }
    }

    pub fn start(&self) {
        self.runtime.clone().run_main_thread(&self.main_class);
    }

    /// Join and wait for the main thread.
    pub fn join(&self) {
        self.runtime.main_thread()
            .expect("can not join main thread - not started")
            .join();
    }

    // pub fn attach_debugger(&self) -> Debugger {
    //     Debugger {
    //         runtime: self.runtime
    //     }
    // }
}

pub enum VmEvent {
    ClassLoad(ClassName),
    FramePush,
    FramePop,
    PcChange,
    LocalsChange,
    OperandStackChange,
    InstanceFieldAccess(ClassName, FieldSig),
    StaticFieldAccess(ClassName, FieldSig),
    NewInstance(InstanceId),
    Error(ExecError)
}

pub struct Debugger {
    runtime: Arc<Runtime>
}

// impl Debugger {
//     pub fn recv_event() -> E
// }

pub trait VmEventHandler {

}