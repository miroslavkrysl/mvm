
use gtk::{prelude::*, NONE_ADJUSTMENT, ScrolledWindow, Viewport, ListBox, Widget, Label, Box, ListBoxRow, Separator, Orientation, Frame, Justification};

pub struct FrameStackView {
    widget: ScrolledWindow
}

impl FrameStackView {
    pub fn new() -> Self {
        let scrolled = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        scrolled.set_min_content_width(200);

        let view_port = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        
        let list_box= ListBox::new();
        
        list_box.add(FrameView::new().widget());
        list_box.add(FrameView::new().widget());
        list_box.add(FrameView::new().widget());
        list_box.add(FrameView::new().widget());
        list_box.add(FrameView::new().widget());

        view_port.add(&list_box);
        
        scrolled.add(&view_port);

        FrameStackView {
            widget: scrolled
        }
    }

    pub fn widget(&self) -> &impl IsA<Widget> {
        &self.widget
    }
}

pub struct FrameView {
    widget: ListBoxRow
}

impl FrameView {
    pub fn new() -> Self {
        let content = Box::new(Orientation::Vertical, 5);
        let method_name = Label::new(Some("ClassName:methodName"));
        method_name.set_justify(Justification::Left);

        let pc_box = Box::new(Orientation::Horizontal, 5);
        let pc_label = Label::new(Some("pc:"));
        let pc_value = Label::new(Some("12"));
        pc_box.pack_start(&pc_label, false, false, 0);
        pc_box.pack_start(&pc_value, false, false, 0);
        
        
        content.pack_start(&method_name, false, false, 0);
        content.pack_start(&pc_box, false, false, 0);
    
        let row = ListBoxRow::new();
        row.get_style_context().add_class("list-box-row");
        row.add(&content);

        FrameView {
            widget: row
        }
    }

    pub fn widget(&self) -> &impl IsA<Widget> {
        &self.widget
    }
}

