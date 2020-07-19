use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::vm::exec::vm::Vm;
use crate::vm::memory::frame::Frame;
use crate::vm::memory::frame_stack::FrameStack;
use crate::vm::exec::error::{RuntimeError, ExecError};
use crate::vm::class::signature::MethodSig;
use crate::vm::class::name::ClassName;


enum ThreadCmd {
    NextStep,
    Stop,
}


pub struct Thread {
    start_method: (ClassName, MethodSig),
    runtime: Arc<Vm>,
    stack: FrameStack,
    join_handle: Mutex<Option<JoinHandle<()>>>,
    cmd_rx: Mutex<Receiver<ThreadCmd>>,
    cmd_tx: Mutex<Sender<ThreadCmd>>,
}


impl Thread {
    pub fn new(runtime: Arc<Vm>, class_name: ClassName, method_sig: MethodSig) -> Arc<Self> {
        let (tx, rx) = channel();

        let mut thread = Arc::new(Thread {
            runtime,
            start_method: (class_name, method_sig),
            stack: FrameStack::new(),
            join_handle: Mutex::new(None),
            cmd_rx: Mutex::new(rx),
            cmd_tx: Mutex::new(tx),
        });

        let join_handle = thread::spawn({
            let thread = thread.clone();
            move || {
                thread.run();
            }
        });

        *thread.join_handle.lock().unwrap() = Some(join_handle);
        thread
    }

    pub fn next_step(&self) {
        self.cmd_tx.lock().unwrap().send(ThreadCmd::NextStep).unwrap();
    }

    pub fn cancel(&self) {
        self.cmd_tx.lock().unwrap().send(ThreadCmd::Stop).unwrap();
    }

    pub fn join(&self) {
        let handle = self.join_handle.lock().unwrap().take();

        match handle {
            None => panic!("thread already joined"),
            Some(handle) => {
                handle.join().unwrap();
            }
        }
    }

    fn run(&self) {
        let (class_name, method_sig) = self.start_method.clone();

        let class = match self.runtime.resolve_class(&class_name) {
            Ok(class) => class,
            Err(error) => {
                self.runtime.notify_error(error.into());
                return;
            },
        };

        let method = match class.static_method(&method_sig) {
            Ok(class) => class.clone(),
            Err(error) => {
                self.runtime.notify_error(error.into());
                return;
            },
        };

        assert!(method.is_static());
        assert!(method.signature().params_desc().is_empty());
        assert!(method.signature().return_desc().is_void());

        let frame = Frame::new(class.clone(), method.clone());
        
        self.stack.push(frame);

        loop {
            match self.cmd_rx.lock().unwrap().recv() {
                Ok(ThreadCmd::NextStep) => {}
                Ok(ThreadCmd::Stop) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }

            let instruction = match self.stack.current() {
                None => {
                    // start method returned
                    break;
                }
                Some(frame) => {
                    match frame.method().code().instruction(frame.pc()) {
                        Ok(instruction) => {
                            instruction
                        }
                        Err(error) => {
                            // probably pc out of bounds
                            self.runtime.notify_error(error.into());
                            return;
                        }
                    }
                }
            };

            if let Err(error) = instruction.execute(&self) {
                // error while executing instruction
                self.runtime.notify_error(error.into());
                return;
            }

            self.runtime.notify_update();
        }
        self.runtime.notify_end();
    }
}


impl Thread {
    pub fn stack(&self) -> &FrameStack {
        &self.stack
    }
    
    pub fn runtime(&self) -> &Arc<Vm> {
        &self.runtime
    }
}