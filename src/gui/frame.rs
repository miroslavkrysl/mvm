use crate::vm::class::{
    name::{ClassName, MethodName},
    signature::MethodSig,
};
use gtk::{
    Align, Box, BoxExt, ContainerExt, Label, ListBoxRow, Orientation, StyleContextExt, WidgetExt,
};
use relm::{Relm, Update, Widget};
use relm_derive::Msg;
use std::boxed::Box as StdBox;

pub struct FrameModel {
    class_name: ClassName,
    method_sig: MethodSig,
}

pub struct FrameView {
    root: ListBoxRow,
}

impl Update for FrameView {
    type Model = FrameModel;
    type ModelParam = (ClassName, MethodSig);
    type Msg = ();

    fn model(_: &Relm<Self>, params: (ClassName, MethodSig)) -> FrameModel {
        FrameModel {
            class_name: params.0,
            method_sig: params.1,
        }
    }

    fn update(&mut self, _: ()) {}
}

impl Widget for FrameView {
    type Root = ListBoxRow;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: FrameModel) -> Self {
        let ret = Label::new(Some(&model.method_sig.return_desc().to_string()));
        ret.set_halign(Align::Start);

        let class = Label::new(Some(&model.class_name.to_string()));
        class.get_style_context().add_class("frame-name");

        let colon = Label::new(Some(":"));
        colon.get_style_context().add_class("frame-name");

        let method = Label::new(Some(&model.method_sig.name().to_string()));
        method.get_style_context().add_class("frame-name");

        let name_box = Box::new(Orientation::Horizontal, 3);
        name_box.pack_start(&class, false, false, 0);
        name_box.pack_start(&colon, false, false, 0);
        name_box.pack_start(&method, false, false, 0);

        let params = Label::new(Some("(int, long, double)"));
        params.set_halign(Align::Start);

        let frame = Box::new(Orientation::Vertical, 5);
        frame.set_property_margin(5);
        frame.pack_start(&ret, false, false, 0);
        frame.pack_start(&name_box, false, false, 0);
        frame.pack_start(&params, false, false, 0);

        let root = ListBoxRow::new();
        root.add(&frame);
        root.show_all();

        FrameView { root }
    }
    fn init_view(&mut self) {}
    fn on_add<W: relm::IsA<gtk::Widget> + relm::IsA<relm::Object>>(&self, _parent: W) {}
    fn parent_id() -> Option<&'static str> {
        None
    }
    fn run(model_param: Self::ModelParam) -> Result<(), ()>
    where
        Self: 'static,
    {
        relm::run::<Self>(model_param)
    }
}
