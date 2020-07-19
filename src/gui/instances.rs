use crate::vm::class::{name::ClassName, signature::MethodSig};
use gtk::{
    Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox,
    ListBoxExt, ListBoxRow, ListBoxRowExt, Orientation, ScrolledWindow, Separator, ShadowType,
    StyleContextExt, Viewport, WidgetExt, NONE_ADJUSTMENT,
};
use relm::{connect, Component, Relm, Update, Widget};
use relm_derive::Msg;
use std::boxed::Box as StdBox;
use std::sync::Arc;

use crate::vm::class::class::Class;
use crate::vm::class::instance::Instance;


#[derive(Msg)]
pub enum InstancesMsg {
    Update(Vec<Instance>),
    InstanceActivated(Instance),
    RowActivated(usize)
}

pub struct InstancesModel {
    classes: Vec<Instance>,
}

pub struct InstancesView {
    root: Box,
    relm: Relm<InstancesView>,
    model: InstancesModel,
    list_view: ListBox,
}

impl Update for InstancesView {
    type Model = InstancesModel;
    type ModelParam = ();
    type Msg = InstancesMsg;

    fn model(_: &Relm<Self>, _: ()) -> InstancesModel {
        InstancesModel {
            classes: Vec::new()
        }
    }

    fn update(&mut self, event: InstancesMsg) {
        match event {
            InstancesMsg::Update(instances) => {
                for row in self.list_view.get_children() {
                    self.list_view.remove(&row);
                }
                self.model.classes.clear();

                for instance in instances {
                    let row = InstanceRow::new(&instance);
                    self.list_view.add(&row.root);
                    self.model.classes.push(instance);
                }
            },
            InstancesMsg::InstanceActivated(class) => {
                // just to notify listeners
            },
            InstancesMsg::RowActivated(index) => {
                self.relm.stream().emit(InstancesMsg::InstanceActivated(self.model.classes[index].clone()));
            },
        }
    }
}

impl Widget for InstancesView {
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
            InstancesMsg::RowActivated(row.get_index() as usize)
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

        let label = Label::new(Some("Instances"));
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        InstancesView {
            root,
            relm: relm.clone(),
            model,
            list_view: list,
        }
    }
}

struct InstanceRow {
    root: ListBoxRow
}

impl InstanceRow {
    fn new(instance: &Instance) -> InstanceRow {
        let instance_id_str = instance.id().to_string();
        let class_name_str = instance.class().name().to_string();

        let id_label = Label::new(Some(&instance_id_str));
        let at_label = Label::new(Some("@"));
        let class_label = Label::new(Some(&class_name_str));
        class_label.get_style_context().add_class("frame-name");

        let instance_box = Box::new(Orientation::Horizontal, 3);
        instance_box.set_property_margin(5);
        instance_box.pack_start(&class_label, false, false, 0);
        instance_box.pack_start(&at_label, false, false, 0);
        instance_box.pack_start(&id_label, false, false, 0);

        let root = ListBoxRow::new();
        root.add(&instance_box);
        root.show_all();

        InstanceRow { root }
    }
}