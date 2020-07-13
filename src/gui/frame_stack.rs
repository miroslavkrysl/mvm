use super::frame::FrameView;
use crate::vm::class::{
    name::{ClassName, MethodName},
    signature::MethodSig,
};
use gtk::{
    Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox,
    ListBoxExt, ListBoxRow, ListBoxRowExt, Orientation, ScrolledWindow, Separator, ShadowType,
    StyleContextExt, Viewport, WidgetExt, NONE_ADJUSTMENT,
};
use relm::{create_component, init, Component, Relm, Update, Widget};
use relm_derive::Msg;
use std::boxed::Box as StdBox;

#[derive(Msg)]
pub enum FrameStackEvent {
    Push(ClassName, MethodSig),
    Pop,
}

pub struct FrameStackView {
    model: (),
    root: Box,
    list: ListBox,
    frames: Vec<Component<FrameView>>,
}

impl Update for FrameStackView {
    type Model = ();
    type ModelParam = ();
    type Msg = FrameStackEvent;

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, event: FrameStackEvent) {
        match event {
            FrameStackEvent::Push(class, method) => {
                println!("push");
                let row = create_component::<FrameView>((class, method));
                self.list.add(row.widget());
                self.frames.push(row);
            }
            FrameStackEvent::Pop => {
                if let Some(f) = self.frames.pop() {
                    self.list.remove(f.widget());
                }
            }
        }
    }
}

impl Widget for FrameStackView {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let list = ListBox::new();
        list.set_header_func(Some(StdBox::new(
            |row: &ListBoxRow, before: Option<&ListBoxRow>| {
                if row.get_header().is_none() && before.is_some() {
                    row.set_header(Some(&Separator::new(Orientation::Horizontal)));
                }
            },
        )));

        let placeholder = Label::new(Some("EMPTY"));
        placeholder
            .get_style_context()
            .add_class("placeholder-text");
        placeholder.set_property_margin(5);
        placeholder.show_all();
        list.set_placeholder(Some(&placeholder));

        let frame = Frame::new(None);
        frame.set_shadow_type(ShadowType::In);
        frame.set_valign(Align::Start);
        frame.add(&list);

        let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        viewport.add(&frame);
        viewport.set_border_width(10);

        let scrolled = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        scrolled.add(&viewport);

        let label = Label::new(Some("Frame stack"));
        label.get_style_context().add_class("section-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        FrameStackView {
            model,
            root,
            list,
            frames: Vec::new(),
        }
    }
}
