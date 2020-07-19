use std::boxed::Box as StdBox;
use std::sync::Arc;

use gtk::{Align, Box, BoxExt, CellLayoutExt, CellRendererText, ContainerExt, Frame, FrameExt, GtkListStoreExt, Justification, Label, LabelExt, ListBox, ListBoxExt, ListBoxRow, ListBoxRowExt, ListStore, NONE_ADJUSTMENT, Orientation, ScrolledWindow, SelectionMode, Separator, ShadowType, StyleContextExt, TreeSelectionExt, TreeView, TreeViewColumn, TreeViewColumnExt, TreeViewExt, TreeViewGridLines, Viewport, WidgetExt};
use gtk::prelude::{GtkListStoreExtManual, StaticType};
use relm::{Component, connect, Relm, Update, Widget};
use relm_derive::Msg;

use crate::vm::class::instance::{InstanceId, Instance};
use crate::vm::class::class::Class;
use crate::vm::class::field::Field;


#[derive(Msg)]
pub enum FieldsMsg {
    Update,
    ChangeViewed(Viewed)
}

pub enum Viewed {
    Class(Arc<Class>),
    Instance(Instance),
    None
}

pub struct FieldsModel {
    viewed: Viewed,
}

pub struct FieldsView {
    root: Box,
    model: FieldsModel,
    relm: Relm<FieldsView>,
    list_store: ListStore,
    tree_view: TreeView,
}


impl Update for FieldsView {
    type Model = FieldsModel;
    type ModelParam = ();
    type Msg = FieldsMsg;

    fn model(_: &Relm<Self>, _: ()) -> FieldsModel {
        FieldsModel {
            viewed: Viewed::None
        }
    }

    fn update(&mut self, event: FieldsMsg) {
        match event {
            FieldsMsg::Update => {
                let fields = match &self.model.viewed {
                    Viewed::Class(class) => {
                        class.fields()
                            .filter(|f| f.is_static())
                            .map(|f: &Arc<Field>| {
                                let sig = f.signature();
                                (sig, class.static_field_value(sig).unwrap())
                            }).collect::<Vec<_>>()
                    },
                    Viewed::Instance(instance) => {
                        instance.class().fields()
                             .filter(|f| !f.is_static())
                             .map(|f: &Arc<Field>| {
                                 let sig = f.signature();
                                 (sig, instance.class().instance_field_value(&instance, sig).unwrap())
                             }).collect::<Vec<_>>()
                    },
                    Viewed::None => Vec::new(),
                };

                self.list_store.clear();

                for (sig, value) in fields {
                    self.list_store.insert_with_values(None,
                                                       &[0, 1, 2],
                                                       &[&sig.type_desc().to_string(),
                                                           &sig.name().to_string(),
                                                           &value.to_string()]);
                }
            },
            FieldsMsg::ChangeViewed(viewed) => {
                self.model.viewed = viewed;
                self.relm.stream().emit(FieldsMsg::Update);
            },
        }
    }
}


impl Widget for FieldsView {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let tree_view = gtk::TreeView::new();

        let type_column = gtk::TreeViewColumn::new();
        type_column.set_title("type");
        let type_cell = gtk::CellRendererText::new();
        type_column.pack_start(&type_cell, true);
        type_column.set_resizable(true);
        type_column.add_attribute(&type_cell, "text", 0);
        tree_view.append_column(&type_column);

        let name_column = gtk::TreeViewColumn::new();
        name_column.set_title("name");
        let name_cell = gtk::CellRendererText::new();
        name_column.pack_start(&name_cell, true);
        name_column.set_resizable(true);
        name_column.add_attribute(&name_cell, "text", 1);
        tree_view.append_column(&name_column);

        let value_column = gtk::TreeViewColumn::new();
        value_column.set_title("value");
        let value_cell = gtk::CellRendererText::new();
        value_column.pack_start(&value_cell, true);
        value_column.add_attribute(&value_cell, "text", 2);
        tree_view.append_column(&value_column);

        let list_store = gtk::ListStore::new(&[String::static_type(), String::static_type(), String::static_type()]);

        tree_view.set_model(Some(&list_store));
        tree_view.set_grid_lines(TreeViewGridLines::Both);
        tree_view.get_selection().set_mode(SelectionMode::None);

        let frame = Frame::new(None);
        frame.set_shadow_type(ShadowType::In);
        frame.set_valign(Align::Start);
        frame.add(&tree_view);

        let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        viewport.add(&frame);
        viewport.set_border_width(10);

        let scrolled = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        scrolled.add(&viewport);

        let label = Label::new(Some("Fields"));
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        FieldsView {
            root,
            model,
            relm: relm.clone(),
            list_store,
            tree_view,
        }
    }
}