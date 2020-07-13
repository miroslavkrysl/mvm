use std::time::Duration;

pub mod class;
pub mod exec;
pub mod memory;
// pub mod vm;
pub mod instruction;
pub mod parse;
pub mod types;

pub struct VirtualMachine {
    // main_class: ClassLoader,
    // runtime: Arc<Runtime>,
    observer: Option<Box<dyn Fn(ItemEvent)>>,
    item: String,
}

impl VirtualMachine {
    pub fn new(s: String) -> Self {
        VirtualMachine {
            observer: None,
            item: s,
        }
    }

    pub fn next(&self) {
        self.fire_event(ItemEvent::EventA(self.item.clone()));
    }
}

impl VirtualMachine {
    pub fn watch<F>(&mut self, callback: F)
    where
        F: 'static + Fn(ItemEvent),
    {
        self.observer = Some(Box::new(callback));
    }

    fn fire_event(&self, event: ItemEvent) {
        match &self.observer {
            Some(o) => o(event),
            None => {}
        }
    }
}

pub enum ItemEvent {
    EventA(String),
    EventB(String),
}

// pub enum FrameStackEvent<'a> {
//     FrameStackPush(&'a Frame),
//     FrameStackPop(&'a Frame),
// }
//
//
// pub enum FrameStackEvent {
//     FrameStackPush,
//     FrameStackPop,
// }
