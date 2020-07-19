use crate::vm::class::{name::ClassName, signature::MethodSig};
use gtk::{
    Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox,
    ListBoxExt, ListBoxRow, ListBoxRowExt, Orientation, ScrolledWindow, Separator, ShadowType,
    StyleContextExt, Viewport, WidgetExt, NONE_ADJUSTMENT,
};
use relm::{connect, create_component, Component, Relm, Update, Widget};
use relm_derive::Msg;
use std::boxed::Box as StdBox;
use std::sync::Arc;

use crate::vm::memory::frame::Frame as VmFrame;


#[derive(Msg)]
pub enum FrameStackMsg {
    Update(Vec<Arc<VmFrame>>),
    FrameActivated(Arc<VmFrame>),
    RowActivated(usize),
    SelectTopFrame,
}

pub struct FrameStackModel {
    frames: Vec<Arc<VmFrame>>,
}

pub struct FrameStackView {
    root: Box,
    relm: Relm<FrameStackView>,
    model: FrameStackModel,
    list_view: ListBox,
}

impl Update for FrameStackView {
    type Model = FrameStackModel;
    type ModelParam = ();
    type Msg = FrameStackMsg;

    fn model(_: &Relm<Self>, _: ()) -> FrameStackModel {
        FrameStackModel {
            frames: Vec::new()
        }
    }

    fn update(&mut self, event: FrameStackMsg) {
        match event {
            FrameStackMsg::Update(frames) => {
                for row in self.list_view.get_children() {
                    self.list_view.remove(&row);
                }
                self.model.frames.clear();

                for frame in frames {
                    let row = FrameStackRow::new(&frame);
                    self.list_view.add(&row.root);
                    self.model.frames.push(frame);
                }
            },
            FrameStackMsg::FrameActivated(frame) => {
                // just to notify listeners
            },
            FrameStackMsg::RowActivated(index) => {
                self.relm.stream().emit(FrameStackMsg::FrameActivated(self.model.frames[index].clone()));
            },
            FrameStackMsg::SelectTopFrame => {
                if let Some(row) = self.list_view.get_row_at_index(0) {
                    self.list_view.select_row(Some(&row));
                }
            },
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
        connect!(
            relm,
            list,
            connect_row_activated(_, row),
            FrameStackMsg::RowActivated(row.get_index() as usize)
        );

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
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        FrameStackView {
            root,
            model,
            relm: relm.clone(),
            list_view: list,
        }
    }
}

struct FrameStackRow {
    root: ListBoxRow
}

impl FrameStackRow {
    fn new(frame: &VmFrame) -> FrameStackRow {
        let method = frame.method();
        let method_ret_str = method.signature().return_desc().to_string();
        let method_name_str = method.signature().name().to_string();
        let method_params_str = method.signature().params_desc().to_string();
        let class_name_str = frame.class().name().to_string();

        let ret = Label::new(Some(&method_ret_str));
        ret.set_halign(Align::Start);

        let class = Label::new(Some(&class_name_str));
        class.get_style_context().add_class("frame-name");

        let colon = Label::new(Some(":"));
        colon.get_style_context().add_class("frame-name");

        let method = Label::new(Some(&method_name_str));
        method.get_style_context().add_class("frame-name");

        let name_box = Box::new(Orientation::Horizontal, 3);
        name_box.pack_start(&class, false, false, 0);
        name_box.pack_start(&colon, false, false, 0);
        name_box.pack_start(&method, false, false, 0);

        let params = Label::new(Some(&method_params_str));
        params.set_halign(Align::Start);

        let frame = Box::new(Orientation::Vertical, 5);
        frame.set_property_margin(5);
        frame.pack_start(&ret, false, false, 0);
        frame.pack_start(&name_box, false, false, 0);
        frame.pack_start(&params, false, false, 0);

        let root = ListBoxRow::new();
        root.add(&frame);
        root.show_all();

        FrameStackRow { root }
    }
}
