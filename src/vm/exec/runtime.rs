use crate::vm::exec::class_loader::ClassLoader;
use crate::vm::memory::heap::Heap;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use crate::vm::exec::vm::VmEvent;
use bus::{Bus, BusReader};
use crate::vm::exec::thread::Thread;
use crate::vm::class::symbolic::MethodRef;
use crate::vm::class::class::Class;
use crate::vm::class::method::Method;
use crate::vm::class::name::{ClassName, MethodName};
use crate::vm::exec::error::{RuntimeError, ClassLoadError, ExecError};
use std::collections::HashMap;
use crate::vm::class::instance::{InstanceId, InstancePtr};
use crate::vm::class::signature::MethodSig;
use crate::vm::class::descriptor::{ReturnDesc, ParamsDesc};


pub struct Runtime {
    class_heap: Mutex<HashMap<ClassName, Arc<Class>>>,
    object_heap: Mutex<HashMap<InstanceId, InstancePtr>>,
    class_loader: ClassLoader,
    main_thread: Mutex<Option<Arc<Thread>>>,
    event_bus: Mutex<Bus<VmEvent>>
}


impl Runtime {
    pub const EVENT_BUS_CAPACITY: usize = 50;

    pub fn new(class_path: Vec<PathBuf>) -> Self {
        Runtime {
            class_heap: Mutex::new(HashMap::new()),
            object_heap: Mutex::new(HashMap::new()),
            class_loader: ClassLoader::new(class_path),
            main_thread: Mutex::new(None),
            event_bus: Mutex::new(Bus::new(Self::EVENT_BUS_CAPACITY))
        }
    }

    pub fn run_main_thread(self: Arc<Self>, class_name: &ClassName) {
        let mut main_thread = self.main_thread.lock().unwrap();

        let class = match self.resolve_class(class_name) {
            Ok(class) => class,
            Err(error) => {
                self.emit_event(VmEvent::Error(error.into()));
                return;
            },
        };

        let method_sig = MethodSig::new(
            ReturnDesc::Void,
            MethodName::new("main").unwrap(),
            ParamsDesc::empty()
        ).unwrap();

        let method = match class.static_method(&method_sig) {
            Ok(class) => class.clone(),
            Err(error) => {
                self.emit_event(VmEvent::Error(ExecError::MainMethodNotFound {
                    class_name: class_name.clone()
                }));
                return;
            },
        };

        let thread = match *main_thread {
            None => Thread::new(self.clone(), class, method),
            Some(_) => panic!("main thread already running"),
        };

        *main_thread = Some(thread);
    }

    pub fn main_thread(&self) -> Option<Arc<Thread>> {
        self.main_thread.lock().unwrap().as_ref().cloned()
    }

    pub fn resolve_class(&self, name: &ClassName) -> Result<Arc<Class>, ClassLoadError> {
        let heap = self.class_heap.lock().unwrap();
        // match heap.class(name) {
        //     Some(class) => {}
        //     
        // }

        unimplemented!()
    }

    pub fn create_instance(&self, class: Arc<Class>) -> Result<Arc<Class>, RuntimeError> {
        unimplemented!()
    }
}

impl Runtime {
    pub fn add_event_listener(&mut self) -> BusReader<VmEvent> {
        self.event_bus.lock().unwrap().add_rx()
    }

    pub fn emit_event(&self, event: VmEvent) {
        self.event_bus.lock().unwrap().broadcast(event);
    }
}

