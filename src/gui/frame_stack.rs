use crate::vm::class::name::{ClassName, MethodName};
use gtk::{
    Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox,
    ListBoxExt, ListBoxRow, ListBoxRowExt, Orientation, ScrolledWindow, Separator, ShadowType,
    StyleContextExt, Viewport, ViewportExt, WidgetExt, NONE_ADJUSTMENT,
};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;
use std::boxed::Box as StdBox;

pub struct FrameStackModel {
    frames: Vec<FrameInfo>,
}

#[derive(Msg)]
pub enum FrameStackEvent {
    Push(FrameInfo),
    Pop,
    ChangePc(usize),
}

pub struct FrameStackView {
    model: FrameStackModel,
    root: Box,
    list: ListBox,
    frames: Vec<Box>,
}

impl Update for FrameStackView {
    type Model = FrameStackModel;
    type ModelParam = ();
    type Msg = FrameStackEvent;

    fn model(_: &Relm<Self>, _: ()) -> FrameStackModel {
        FrameStackModel { frames: Vec::new() }
    }

    fn update(&mut self, event: FrameStackEvent) {
        match event {
            FrameStackEvent::Push(info) => {
                self.model.frames.push(info);
                let frame = Box::new(Orientation::Horizontal, 0);
                let label = Label::new(Some("hello"));
                label.set_visible(true);
                frame.pack_start(&label, false, false, 5);
                frame.set_visible(true);
                self.list.add(&frame);
                self.frames.push(frame);
            }
            FrameStackEvent::Pop => {
                if let Some(f) = self.model.frames.pop() {
                    self.list.remove(&self.frames.pop().unwrap());
                }
            }
            FrameStackEvent::ChangePc(pc) => {
                if let Some(f) = self.frames.last() {
                    // f.set_label(&pc.to_string());
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

pub struct FrameInfo {
    pub class: ClassName,
    pub method: MethodName,
    pub pc: usize,
}
