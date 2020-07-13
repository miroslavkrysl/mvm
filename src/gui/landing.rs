use gtk::{
    Align, BaselinePosition, Box, BoxExt, IconSize, Image, ImageExt, Label, Orientation,
    StyleContextExt, WidgetExt,
};
use relm::{Relm, Update, Widget};

pub struct LandingPage {
    root: Box,
}

impl Update for LandingPage {
    type Model = ();
    type ModelParam = ();
    type Msg = ();

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, _: ()) {}
}

impl Widget for LandingPage {
    type Root = Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(_: &Relm<Self>, _: Self::Model) -> Self {
        let image = Image::from_icon_name(Some("computer-symbolic"), IconSize::Invalid);
        image.set_pixel_size(200);

        let text = Label::new(Some("Visual interpreter of Java bytecode subset."));
        text.set_widget_name("welcome-text");

        let inner = Box::new(Orientation::Vertical, 30);
        inner.set_baseline_position(BaselinePosition::Center);
        inner.set_property_margin(20);
        inner.set_halign(Align::Center);
        inner.set_valign(Align::Center);

        inner.pack_start(&image, false, false, 0);
        inner.pack_start(&text, false, false, 0);

        let root = Box::new(Orientation::Vertical, 0);
        root.pack_start(&inner, true, true, 0);

        LandingPage { root }
    }
}
