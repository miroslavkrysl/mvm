use std::sync::{Arc, Condvar, Mutex};
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::vm::class::class::Class;
use crate::vm::class::error::CodeError;
use crate::vm::class::method::Method;
use crate::vm::exec::runtime::Runtime;
use crate::vm::exec::vm::VmEvent;
use crate::vm::memory::frame::Frame;
use crate::vm::memory::frame_stack::FrameStack;
use crate::vm::exec::error::RuntimeError;


enum ThreadCmd {
    NextStep,
    Stop,
}


pub struct Thread {
    start_method: (Arc<Class>, Arc<Method>),
    runtime: Arc<Runtime>,
    stack: FrameStack,
    join_handle: Mutex<Option<JoinHandle<()>>>,
    cmd_rx: Mutex<Receiver<ThreadCmd>>,
    cmd_tx: Mutex<Sender<ThreadCmd>>,
}


impl Thread {
    pub fn new(runtime: Arc<Runtime>, class: Arc<Class>, method: Arc<Method>) -> Arc<Self> {
        assert!(method.is_static());
        assert!(method.signature().params_desc().is_empty());
        assert!(method.signature().return_desc().is_void());

        let (tx, rx) = channel();

        let mut thread = Arc::new(Thread {
            runtime,
            start_method: (class, method),
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
        let (class, method) = self.start_method.clone();
        let frame = Frame::new(class.clone(), method.clone());
        
        self.stack.push(frame);
        self.runtime.emit_event(VmEvent::FramePush);

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
                            self.runtime.emit_event(VmEvent::Error(error.into()));
                            break;
                        }
                    }
                }
            };

            if let Err(error) = instruction.execute(&self) {
                // error while executing instruction
                self.runtime.emit_event(VmEvent::Error(error.into()));
            }
        }
    }
}


impl Thread {
    pub fn stack(&self) -> &FrameStack {
        &self.stack
    }
    
    pub fn runtime(&self) -> &Arc<Runtime> {
        &self.runtime
    }
}