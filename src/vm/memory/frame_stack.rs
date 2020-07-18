use std::sync::{Arc, Mutex};
use crate::vm::memory::frame::Frame;

pub struct FrameStack {
    frames: Mutex<Vec<Arc<Frame>>>
}

impl FrameStack {
    pub fn new() -> Self {
        FrameStack {
            frames: Mutex::new(Vec::new())
        }
    }

    pub fn push(&self, frame: Frame) {
        self.frames.lock().unwrap().push(Arc::new(frame))
    }

    pub fn current(&self) -> Option<Arc<Frame>> {
        self.frames.lock().unwrap().last().cloned()
    }

    pub fn pop(&self) -> Option<Arc<Frame>> {
        self.frames.lock().unwrap().pop()
    }
}