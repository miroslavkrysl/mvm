use crate::class::ClassName;
use crate::error::VmError;
use std::sync::Arc;
use crate::exec::Runtime;
// use crate::exec::Thread;

pub struct VirtualMachine {
    main_class: ClassName,
    runtime: Arc<Runtime>,
}

impl VirtualMachine {
    pub fn new(main_class: ClassName) -> Self {
        VirtualMachine {
            main_class,
            runtime: Runtime::new()
        }
    }

    pub fn start(&self) -> Result<(), VmError> {
        // let thread = Arc::new(Thread::new());
    }
}