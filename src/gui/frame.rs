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