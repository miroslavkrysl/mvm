use std::path::PathBuf;

use gtk::{BoxExt, Button, ContainerExt, Dialog, DialogExt, EditableSignals, Entry, EntryExt, FileChooserAction, FileChooserButton, FileChooserButtonExt, FileChooserExt, Grid, GridExt, GtkWindowExt, Inhibit, Label, ResponseType, StyleContextExt, WidgetExt, Window, WindowType};
use gtk::prelude::Cast;
use relm::{
    Component, connect, create_component, Relm, Update,
    Widget,
};
use relm_derive::Msg;

use crate::gui::vm::{VmMsg, VmView};
use crate::vm::{
    class::{
        name::{ClassName},
    },
};

use super::{
    header::AppHeaderEvent,
    header::AppHeaderView,
    landing::LandingPage,
};


#[derive(Msg)]
pub enum AppEvent {
    LoadRequest,
    Quit,
}


pub struct AppWindow {
    window: Window,
    _header: Component<AppHeaderView>,
    load_dialog: LoadDialog,
    vm: Option<Component<VmView>>,
    landing_page: Component<LandingPage>,
}


impl Update for AppWindow {
    type Model = ();
    type ModelParam = ();
    type Msg = AppEvent;

    fn model(_: &Relm<Self>, _: ()) -> () {}

    fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::LoadRequest => {
                let result = self.load_dialog.run();

                if let Some((class_name, path)) = result {
                    if let Some(vm) = &self.vm {
                        vm.emit(VmMsg::Load(class_name, vec![path]));
                    } else {
                        let vm = create_component::<VmView>((class_name, vec![path]));
                        self.window.remove(self.landing_page.widget());
                        self.window.add(vm.widget());
                        self.vm = Some(vm);
                        self.window.maximize();
                    }
                }
            }
            AppEvent::Quit => gtk::main_quit(),
        }
    }
}


impl Widget for AppWindow {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _: Self::Model) -> Self {
        // set up window
        let window = Window::new(WindowType::Toplevel);
        window.set_title("MVM - Mirek's Virtual Machine");
        window.set_default_size(800, 600);
        Window::set_default_icon_name("computer");

        // handle close button
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(AppEvent::Quit), Inhibit(false))
        );

        let header = create_component::<AppHeaderView>(());
        window.set_titlebar(Some(header.widget()));

        let landing_page = create_component::<LandingPage>(());
        window.add(landing_page.widget());

        connect!(
            header@AppHeaderEvent::Load,
            relm,
            AppEvent::LoadRequest
        );

        window.show_all();

        let load_dialog = LoadDialog::new();

        AppWindow {
            window,
            _header: header,
            load_dialog,
            vm: None,
            landing_page,
        }
    }
}


struct LoadDialog {
    dialog: Dialog,
    path_chooser: FileChooserButton,
    class_entry: Entry,
    _load_button: Button,
}


impl LoadDialog {
    fn new() -> LoadDialog {
        let path_label = Label::new(Some("Class path:"));
        let class_label = Label::new(Some("Main class:"));
        let path_chooser = FileChooserButton::new("Class path", FileChooserAction::SelectFolder);
        let class_entry = Entry::new();

        let grid = Grid::new();
        grid.set_column_spacing(5);
        grid.set_row_spacing(5);
        grid.set_property_margin(10);

        grid.attach(&path_label, 0, 0, 1, 1);
        grid.attach(&class_label, 0, 1, 1, 1);
        grid.attach(&path_chooser, 1, 0, 1, 1);
        grid.attach(&class_entry, 1, 1, 1, 1);

        let dialog = Dialog::new();
        dialog.get_content_area().pack_start(&grid, true, true, 0);
        dialog.add_button("Cancel", ResponseType::Cancel);
        let load_button = dialog.add_button("Load", ResponseType::Accept).downcast::<Button>().unwrap();
        load_button.set_sensitive(false);

        let ce = class_entry.clone();
        let pc = path_chooser.clone();
        let lb = load_button.clone();
        class_entry.clone().connect_changed(move |entry| {
            let mut ok = true;

            if ClassName::new(ce.get_text()).is_err() {
                entry.get_style_context().add_class("entry-error");
                ok = false;
            } else {
                entry.get_style_context().remove_class("entry-error");
            }

            if pc.get_filename().is_none() {
                ok = false;
            }
            lb.set_sensitive(ok);
        });


        let ce = class_entry.clone();
        let pc = path_chooser.clone();
        let lb = load_button.clone();
        path_chooser.clone().connect_file_set(move |_| {
            let mut ok = true;

            if ClassName::new(ce.get_text()).is_err() {
                ok = false;
            }
            if pc.get_filename().is_none() {
                ok = false;
            }
            lb.set_sensitive(ok);
        });

        LoadDialog {
            dialog,
            path_chooser,
            class_entry,
            _load_button: load_button,
        }
    }

    fn run(&self) -> Option<(ClassName, PathBuf)> {
        self.dialog.show_all();
        loop {
            let result = self.dialog.run();
            if result != ResponseType::Accept {
                self.dialog.hide();
                return None;
            }

            let class = ClassName::new(self.class_entry.get_text().trim()).unwrap();
            let path = self.path_chooser.get_filename().unwrap();

            self.dialog.hide();
            return Some((class, path));
        }
    }
}