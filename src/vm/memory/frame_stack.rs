use std::sync::{Arc, Mutex};

use crate::vm::memory::frame::Frame;


/// A frame stack.
pub struct FrameStack {
    frames: Mutex<Vec<Arc<Frame>>>
}


impl FrameStack {
    /// Creates a new FrameStack.
    pub fn new() -> Self {
        FrameStack {
            frames: Mutex::new(Vec::new())
        }
    }

    /// Adds the frame to the top.
    pub fn push(&self, frame: Frame) {
        self.frames.lock().unwrap().push(Arc::new(frame))
    }

    /// Gets first frame from the top of the stack.
    pub fn current(&self) -> Option<Arc<Frame>> {
        self.frames.lock().unwrap().last().cloned()
    }

    /// Remove top frame from the stack.
    pub fn pop(&self) -> Option<Arc<Frame>> {
        self.frames.lock().unwrap().pop()
    }

    /// Get all frames. The the first added is first and the last added is last.
    pub fn frames(&self) -> Vec<Arc<Frame>> {
        self.frames.lock().unwrap().clone()
    }
}