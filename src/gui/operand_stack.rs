use gtk::{Align, Box, BoxExt, CellLayoutExt, ContainerExt, Frame, FrameExt, GtkListStoreExt, Justification, Label, LabelExt, ListStore, NONE_ADJUSTMENT, Orientation, ScrolledWindow, SelectionMode, ShadowType, StyleContextExt, TreeSelectionExt, TreeView, TreeViewColumnExt, TreeViewExt, TreeViewGridLines, Viewport, WidgetExt};
use gtk::prelude::{GtkListStoreExtManual, StaticType};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;

use crate::vm::types::value::Value;


#[derive(Msg)]
pub enum OperandStackMsg {
    Update(Vec<Value>)
}


pub struct OperandStackView {
    root: Box,
    list_store: ListStore,
    _tree_view: TreeView,
}


impl Update for OperandStackView {
    type Model = ();
    type ModelParam = ();
    type Msg = OperandStackMsg;

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, event: OperandStackMsg) {
        match event {
            OperandStackMsg::Update(values) => {
                self.list_store.clear();

                for value in values {
                    self.list_store.insert_with_values(None,
                                                       &[0, 1, 2],
                                                       &[&value.value_type().category().size().to_string(),
                                                           &value.value_type().to_string(),
                                                           &value.to_string()]);
                }
            }
        }
    }
}


impl Widget for OperandStackView {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(_: &Relm<Self>, _: Self::Model) -> Self {
        let tree_view = gtk::TreeView::new();

        let size_column = gtk::TreeViewColumn::new();
        size_column.set_title("size");
        let size_cell = gtk::CellRendererText::new();
        size_column.pack_start(&size_cell, true);
        size_column.set_resizable(true);
        size_column.add_attribute(&size_cell, "text", 0);
        tree_view.append_column(&size_column);

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

        let label = Label::new(Some("Operand Stack"));
        label.get_style_context().add_class("panel-heading");
        label.set_justify(Justification::Center);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&label, false, false, 10);
        root.pack_start(&scrolled, true, true, 0);
        root.set_size_request(250, -1);

        OperandStackView {
            root,
            list_store,
            _tree_view: tree_view,
        }
    }
}
