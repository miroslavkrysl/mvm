use gtk::{Button, ButtonExt, HeaderBar, HeaderBarExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;


#[derive(Msg)]
pub enum AppHeaderEvent {
    Load,
}


pub struct AppHeaderView {
    header: HeaderBar,
}


impl Update for AppHeaderView {
    type Model = ();
    type ModelParam = ();
    type Msg = AppHeaderEvent;

    fn model(_: &Relm<Self>, _: Self::ModelParam) -> Self::Model {
        ()
    }

    fn update(&mut self, _: Self::Msg) {}
}


impl Widget for AppHeaderView {
    type Root = HeaderBar;

    fn root(&self) -> Self::Root {
        self.header.clone()
    }

    fn view(relm: &Relm<Self>, _: Self::Model) -> Self {
        let header = HeaderBar::new();
        header.set_title(Some("MVM - Mirek's Virtual Machine"));
        header.set_show_close_button(true);

        let load_button = Button::with_label("Load");
        header.pack_start(&load_button);

        connect!(relm, load_button, connect_clicked(_), AppHeaderEvent::Load);

        AppHeaderView { header }
    }
}
