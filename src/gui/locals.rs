use std::boxed::Box as StdBox;
use std::sync::Arc;

use gtk::{Align, Box, BoxExt, ContainerExt, Frame, FrameExt, Justification, Label, LabelExt, ListBox, ListBoxExt, ListBoxRow, ListBoxRowExt, NONE_ADJUSTMENT, Orientation, ScrolledWindow, Separator, ShadowType, StyleContextExt, TreeView, TreeViewExt, Viewport, WidgetExt, TreeViewColumn, CellRendererText, CellLayoutExt, ListStore, GtkListStoreExt, TreeViewColumnExt};
use relm::{Component, connect, Relm, Update, Widget};
use relm_derive::Msg;

use crate::vm::memory::locals::Slot;
use gtk::prelude::{StaticType, GtkListStoreExtManual};
use std::fs;
use std::path::MAIN_SEPARATOR;


#[derive(Msg)]
pub enum LocalsMsg {
    Update(Vec<Slot>)
}


pub struct LocalsView {
    root: Box,
    list_store: ListStore,
    tree_view: TreeView,
}


impl Update for LocalsView {
    type Model = ();
    type ModelParam = ();
    type Msg = LocalsMsg;

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, event: LocalsMsg) {
        match event {
            LocalsMsg::Update(values) => {
                self.list_store.clear();

                for (index, value) in values.iter().enumerate() {
                    let index = index.to_string();
                    match value {
                        Slot::Undefined => {
                            self.list_store.insert_with_values(None,
                                                          &[0, 1, 2],
                                                          &[&index, &"", &"UNDEFINED"]);
                        },
                        Slot::Value(value) => {
                            self.list_store.insert_with_values(None,
                                                               &[0, 1, 2],
                                                               &[&index, &value.value_type().to_string(), &value.to_string()]);
                        },
                    }
                }
            }
        }
    }
}


impl Widget for LocalsView {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let tree_view = gtk::TreeView::new();

        let index_column = gtk::TreeViewColumn::new();
        index_column.set_title("index");
        let index_cell = gtk::CellRendererText::new();
        index_column.pack_start(&index_cell, true);
        index_column.add_attribute(&index_cell, "text", 0);
        tree_view.append_column(&index_column);

        let type_column = gtk::TreeViewColumn::new();
        type_column.set_title("type");
        let type_cell = gtk::CellRendererText::new();
        type_column.pack_start(&type_cell, true);
        type_column.set_resizable(true);
        type_column.add_attribute(&type_cell, "text", 1);
        tree_view.append_column(&type_column);

        let value_column = gtk::TreeViewColumn::new();
        value_column.set_title("value");
        let value_cell = gtk::CellRendererText::new();
        value_column.pack_start(&value_cell, true);
        value_column.add_attribute(&value_cell, "text", 2);
        tree_view.append_column(&value_column);

        let list_store = gtk::ListStore::new(&[String::static_type(), String::static_type(), String::static_type()]);

        tree_view.set_model(Some(&list_store));

        let frame = Frame::new(None);
        frame.set_shadow_type(ShadowType::In);
        frame.set_valign(Align::Start);
        frame.add(&tree_view);

        let viewport = Viewport::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        viewport.add(&frame);
        viewport.set_border_width(10);

        let scrolled = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        scrolled.add(&viewport);

        let label = Label::new(Some("Locals stack"));
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        LocalsView {
            root,
            list_store,
            tree_view,
        }
    }
}
