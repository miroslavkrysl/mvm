use gdk::Screen;
use gtk::{CssProviderExt, STYLE_PROVIDER_PRIORITY_APPLICATION, StyleContext};
use relm::Widget;

use gui::AppWindow;


pub mod gui;
pub mod vm;


fn main() {
    gtk::init().expect("Can not initialize GTK application. Probably missing GTK dependencies.");
    load_css();
    AppWindow::run(()).unwrap();
}


fn load_css() {
    let css_data = include_bytes!("gui/style.css");
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(css_data)
        .expect("Failed to load CSS");

    StyleContext::add_provider_for_screen(
        &Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}