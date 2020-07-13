
use gtk::{prelude::*, Paned, Orientation, Widget};
use super::frame_stack::FrameStackView;

pub struct Content {
    pub widget: Paned,
}


impl Content {
    pub fn new() -> Self {
        let paned = Paned::new(Orientation::Horizontal);

        let frame_stack = FrameStackView::new();
        paned.add1(frame_stack.widget());

        Content {
            widget: paned
        }
    }

    pub fn widget(&self) -> &impl IsA<Widget> {
        &self.widget
    }
}