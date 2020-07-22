use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::vm::class::name::ClassName;
use crate::vm::class::signature::MethodSig;
use crate::vm::exec::vm::Vm;
use crate::vm::memory::frame::Frame;
use crate::vm::memory::frame_stack::FrameStack;


/// An internal command enum for notifying
/// the thread from other thread.
enum ThreadCmd {
    NextStep,
    Stop,
}


/// A virtual machine thread.
/// It runs a system thread.
/// It can be controlled by calling the next and cancel method.
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

        let thread = Arc::new(Thread {
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
            }
        };

        let method = match class.static_method(&method_sig) {
            Ok(class) => class.clone(),
            Err(error) => {
                self.runtime.notify_error(error.into());
                return;
            }
        };

        assert!(method.is_static());
        assert!(method.signature().params_desc().is_empty());
        assert!(method.signature().return_desc().is_void());

        let frame = Frame::new(class.clone(), method.clone());

        self.stack.push(frame);

        loop {
            let instruction = match self.stack.current() {
                None => {
                    // start method returned
                    break;
                }
                Some(frame) => {
                    self.runtime.notify_update();
                    match frame.method().code().instruction(frame.pc()) {
                        Ok(instruction) => {
                            instruction
                        }
                        Err(error) => {
                            // probably pc out of bounds
                            self.runtime.notify_error(error.into());
                            break;
                        }
                    }
                }
            };

            match self.cmd_rx.lock().unwrap().recv() {
                Ok(ThreadCmd::NextStep) => {}
                Ok(ThreadCmd::Stop) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }

            if let Err(error) = instruction.execute(&self) {
                // error while executing instruction
                self.runtime.notify_error(error.into());
                break;
            }
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