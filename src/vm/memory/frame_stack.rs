use std::sync::{Arc, RwLock};
use crate::vm::memory::frame::Frame;

pub struct FrameStack {
    frames: Vec<Frame>
}

impl FrameStack {
    pub fn new() -> Self {
        FrameStack {
            frames: Vec::new()
        }
    }

    pub fn push(&mut self, frame: Frame) {
        self.frames.push(frame)
    }

    pub fn current(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn current_mut(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    pub fn pop(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}