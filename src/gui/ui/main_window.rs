use gtk::{Window, prelude::*, WindowType, main_quit, HeaderBar, ButtonBox, Orientation, Button};
use super::content::Content;

pub struct MainWindow {
    window:  Window
}


impl MainWindow {
    pub fn new() -> MainWindow {
        let window = Window::new(WindowType::Toplevel);

        window.set_title("Mirek's Virtual Machine");
        Window::set_default_icon_name("computer");
        window.maximize();

        let header = Header::new();
        window.set_titlebar(Some(&header.container));

        let content = Content::new();
        window.add(content.widget());

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        window.show_all();

        MainWindow {
            window
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}


pub struct Header {
    pub container: HeaderBar
}

impl Header {
    pub fn new() -> Self {
        let container = HeaderBar::new();


        container.set_title(Some("MVM - Mirek's Virtual Machine"));
        container.set_show_close_button(true);

        let button_box = ButtonBox::new(Orientation::Horizontal);
        let load_button = Button::with_label("Load");
        
        button_box.pack_start(&load_button, false, false, 0);
        container.pack_start(&button_box);

        Header {
            container
        }
    }
}
