use crate::vm::exec::class_loader::ClassLoader;
use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use crate::vm::exec::thread::Thread;
use crate::vm::class::class::Class;
use crate::vm::class::name::{ClassName, MethodName};
use crate::vm::exec::error::{ClassLoadError, ExecError};
use std::collections::HashMap;
use crate::vm::class::instance::{InstanceId, Instance};
use crate::vm::class::signature::MethodSig;
use crate::vm::class::descriptor::{ReturnDesc, ParamsDesc};
use std::ops::{DerefMut, Deref};
use crate::vm::memory::frame::Frame;


pub struct Vm {
    class_heap: Mutex<HashMap<ClassName, Arc<Class>>>,
    object_heap: Mutex<HashMap<InstanceId, Instance>>,
    class_loader: ClassLoader,
    thread: Mutex<Option<Arc<Thread>>>,
    error_callback: Mutex<Option<Box<dyn 'static + Send + FnMut(ExecError)>>>,
    update_callback: Mutex<Option<Box<dyn 'static + Send + FnMut()>>>,
    end_callback: Mutex<Option<Box<dyn 'static + Send + FnMut()>>>
}


impl Vm {
    pub const EVENT_BUS_CAPACITY: usize = 50;

    pub fn new(class_path: Vec<PathBuf>) -> Self {
        Vm {
            class_heap: Mutex::new(HashMap::new()),
            object_heap: Mutex::new(HashMap::new()),
            class_loader: ClassLoader::new(class_path),
            thread: Mutex::new(None),
            error_callback: Mutex::new(None),
            update_callback: Mutex::new(None),
            end_callback: Mutex::new(None)
        }
    }

    pub fn start(self: Arc<Self>, class_name: ClassName) {
        let mut main_thread = self.thread.lock().unwrap();
        let method_sig = MethodSig::new(
            ReturnDesc::Void,
            MethodName::new("main").unwrap(),
            ParamsDesc::empty()
        ).unwrap();

        let thread = match *main_thread {
            None => Thread::new(self.clone(), class_name, method_sig),
            Some(_) => panic!("main thread already running"),
        };

        *main_thread = Some(thread);
    }

    /// Join and wait for the main thread.
    pub fn join(&self) {
        let thread = self.thread.lock().unwrap().deref()
            .clone()
            .expect("can not join main thread - not started")
            .join();

    }

    pub fn thread(&self) -> Option<Arc<Thread>> {
        self.thread.lock().unwrap().as_ref().cloned()
    }

    pub fn resolve_class(&self, name: &ClassName) -> Result<Arc<Class>, ClassLoadError> {
        let mut heap = self.class_heap.lock().unwrap();
        match heap.get(name) {
            Some(class) => {
                Ok(class.clone())
            },
            None => {
                let class = Arc::new(self.class_loader.load(name)?);
                heap.insert(name.clone(), class.clone());
                Ok(class)
            }
        }
    }

    pub fn create_instance(&self, class: Arc<Class>) -> Instance {
        let instance = Instance::new(class);
        self.object_heap.lock().unwrap().insert(instance.id(), instance.clone());
        instance
    }
}

impl Vm {
    pub fn set_error_callback(&self, callback: Option<Box<dyn 'static + Send + FnMut(ExecError)>>) {
        *self.error_callback.lock().unwrap() = callback;
    }

    pub fn set_update_callback(&self, callback: Option<Box<dyn 'static + Send + FnMut()>>) {
        *self.update_callback.lock().unwrap() = callback;
    }

    pub fn set_end_callback(&self, callback: Option<Box<dyn 'static + Send + FnMut()>>) {
        *self.end_callback.lock().unwrap() = callback;
    }

    pub fn notify_error(&self, error: ExecError) {
        let mut callback = self.error_callback.lock().unwrap();
        if let Some(callback) = callback.deref_mut() {
            callback(error)
        }
    }

    pub fn notify_update(&self) {
        let mut callback = self.update_callback.lock().unwrap();
        if let Some(callback) = callback.deref_mut() {
            callback()
        }
    }

    pub fn notify_end(&self) {
        let mut callback = self.end_callback.lock().unwrap();
        if let Some(callback) = callback.deref_mut() {
            callback()
        }
    }
}

impl Vm {
    pub fn classes(&self) -> Vec<Arc<Class>> {
        self.class_heap.lock().unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn instances(&self) -> Vec<Instance> {
        self.object_heap.lock().unwrap()
            .values()
            .cloned()
            .collect()
    }

    pub fn frames(&self) -> Option<Vec<Arc<Frame>>> {
        match self.thread.lock().unwrap().deref() {
            None => None,
            Some(thread) => Some(thread.stack().frames()),
        }
    }

    pub fn next(&self) {
        match self.thread.lock().unwrap().deref_mut() {
            None => {},
            Some(thread) => thread.next_step(),
        }
    }

    pub fn stop(&self) {
        match self.thread.lock().unwrap().deref_mut() {
            None => {},
            Some(thread) => thread.cancel(),
        }
    }
}



