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
use crate::vm::class::class::Class;


#[derive(Msg)]
pub enum ClassesMsg {
    Update(Vec<Arc<Class>>),
    ClassActivated(Arc<Class>),
    RowActivated(usize)
}

pub struct ClassesModel {
    classes: Vec<Arc<Class>>,
}

pub struct ClassesView {
    root: Box,
    relm: Relm<ClassesView>,
    model: ClassesModel,
    list_view: ListBox,
}

impl Update for ClassesView {
    type Model = ClassesModel;
    type ModelParam = ();
    type Msg = ClassesMsg;

    fn model(_: &Relm<Self>, _: ()) -> ClassesModel {
        ClassesModel {
            classes: Vec::new()
        }
    }

    fn update(&mut self, event: ClassesMsg) {
        match event {
            ClassesMsg::Update(classes) => {
                for row in self.list_view.get_children() {
                    self.list_view.remove(&row);
                }
                self.model.classes.clear();

                for class in classes {
                    let row = ClassesRow::new(&class);
                    self.list_view.add(&row.root);
                    self.model.classes.push(class);
                }
            },
            ClassesMsg::ClassActivated(class) => {
                // just to notify listeners
            },
            ClassesMsg::RowActivated(index) => {
                self.relm.stream().emit(ClassesMsg::ClassActivated(self.model.classes[index].clone()));
            },
        }
    }
}

impl Widget for ClassesView {
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
            ClassesMsg::RowActivated(row.get_index() as usize)
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

        let label = Label::new(Some("Classes"));
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        ClassesView {
            root,
            relm: relm.clone(),
            model,
            list_view: list,
        }
    }
}

struct ClassesRow {
    root: ListBoxRow
}

impl ClassesRow {
    fn new(class: &Arc<Class>) -> ClassesRow {
        let class_name_str = class.name().to_string();

        let class_label = Label::new(Some(&class_name_str));
        class_label.get_style_context().add_class("frame-name");

        let frame = Box::new(Orientation::Horizontal, 5);
        frame.set_property_margin(5);
        frame.pack_start(&class_label, false, false, 0);

        let root = ListBoxRow::new();
        root.add(&frame);
        root.show_all();

        ClassesRow { root }
    }
}
