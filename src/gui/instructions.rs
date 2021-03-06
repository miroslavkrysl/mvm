use std::boxed::Box as StdBox;
use std::sync::Arc;

use gtk::{Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox, ListBoxExt, ListBoxRow, ListBoxRowExt, NONE_ADJUSTMENT, Orientation, ScrolledWindow, SelectionMode, Separator, ShadowType, StyleContextExt, Viewport, WidgetExt};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

use crate::vm::bytecode::instruction::Instruction;
use crate::vm::class::class::Class;
use crate::vm::class::method::Method;


#[derive(Msg)]
pub enum InstructionsMsg {
    ChangeViewed(Arc<Class>, Arc<Method>),
    SelectInstruction(isize),
}


pub struct InstructionsView {
    root: Box,
    heading: Label,
    list_view: ListBox,
}


impl Update for InstructionsView {
    type Model = ();
    type ModelParam = ();
    type Msg = InstructionsMsg;

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, event: InstructionsMsg) {
        match event {
            InstructionsMsg::ChangeViewed(class, method) => {
                for row in self.list_view.get_children() {
                    self.list_view.remove(&row);
                }

                for instruction in method.code().instructions() {
                    let row = InstructionsRow::new(&instruction);
                    self.list_view.add(&row.root);
                }

                let method_str = format!("{} {} {} ({})",
                                         method.signature().return_desc().to_string(),
                                         class.name().to_string(),
                                         method.signature().name().to_string(),
                                         method.signature().params_desc().to_string(),
                );
                self.heading.set_label(&method_str);
            }
            InstructionsMsg::SelectInstruction(index) => {
                if let Some(row) = self.list_view.get_row_at_index(index as i32) {
                    self.list_view.select_row(Some(&row));
                }
            }
        }
    }
}


impl Widget for InstructionsView {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(_: &Relm<Self>, _: Self::Model) -> Self {
        let list = ListBox::new();
        list.set_header_func(Some(StdBox::new(
            |row: &ListBoxRow, before: Option<&ListBoxRow>| {
                if row.get_header().is_none() && before.is_some() {
                    row.set_header(Some(&Separator::new(Orientation::Horizontal)));
                }
            },
        )));
        list.set_selection_mode(SelectionMode::Single);

        let frame = Frame::new(None);
        frame.set_shadow_type(ShadowType::In);
        frame.set_valign(Align::Start);
        frame.add(&list);

        let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        viewport.add(&frame);
        viewport.set_border_width(25);

        let scrolled = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        scrolled.add(&viewport);

        let label = Label::new(None);
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        InstructionsView {
            root,
            heading: label,
            list_view: list,
        }
    }
}


struct InstructionsRow {
    root: ListBoxRow
}


impl InstructionsRow {
    fn new(instruction: &Instruction) -> InstructionsRow {
        let instruction_str = instruction.to_string();

        let instruction_label = Label::new(Some(&instruction_str));
        instruction_label.get_style_context().add_class("instruction");

        let instruction_box = Box::new(Orientation::Horizontal, 5);
        instruction_box.set_property_margin(5);
        instruction_box.pack_start(&instruction_label, false, false, 0);

        let root = ListBoxRow::new();
        root.add(&instruction_box);
        root.show_all();

        InstructionsRow { root }
    }
}