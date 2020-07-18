use super::{
    frame_stack::{FrameStackEvent, FrameStackView},
    header::AppHeaderEvent,
    header::AppHeaderView,
    landing::LandingPage,
};
use crate::vm::{
    class::{
        descriptor::{ParamsDesc, ReturnDesc, TypeDesc},
        name::{ClassName, MethodName},
        signature::MethodSig,
    },
    VirtualMachine,
};
use gdk::Screen;
use gtk::{
    Button, ContainerExt, CssProviderExt, Entry, GtkWindowExt, Inhibit, Label, Orientation, Stack,
    StackExt, StackSwitcher, StackSwitcherExt, StyleContext, WidgetExt, Window, WindowType,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use relm::{
    connect, connect_stream, create_component, init, interval, Channel, Component, Relm, Update,
    Widget,
};
use relm_derive::Msg;


pub struct AppState {
    vm: Option<VirtualMachine>,
    ended: bool,
}

#[derive(Msg)]
pub enum AppEvent {
    LoadRequest,
    FrameSelected(usize),
    NextInstruction,
    RestartVm,
    Quit,
}

pub struct AppWindow {
    model: AppState,
    window: Window,
    header: Component<AppHeaderView>,
    content: Stack,
    frame_stack: Component<FrameStackView>,
}

impl Update for AppWindow {
    type Model = AppState;
    type ModelParam = ();
    type Msg = AppEvent;

    fn model(_: &Relm<Self>, _: ()) -> AppState {
        AppState {
            vm: None,
            ended: false,
        }
    }

    fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::LoadRequest => {
                // let params = show_load_dialog(&self.window);
                // self.window.maximize();
                // self.content.set_visible_child_name("vm");
                // println!("Load request");
                self.frame_stack.emit(FrameStackEvent::Push(
                    ClassName::new("hello").unwrap(),
                    MethodSig::new(
                        ReturnDesc::Void,
                        MethodName::new("hello").unwrap(),
                        [TypeDesc::Byte, TypeDesc::Int].iter().cloned().collect(),
                    )
                    .unwrap(),
                ));
            }
            AppEvent::NextInstruction => {}
            AppEvent::RestartVm => {}
            AppEvent::Quit => gtk::main_quit(),
            AppEvent::FrameSelected(i) => {
                println!("{}", i);
            }
        }
    }
}

impl Widget for AppWindow {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
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

        // load CSS
        Self::load_css();

        let header = create_component::<AppHeaderView>(());
        window.set_titlebar(Some(header.widget()));

        let content = Stack::new();

        let frame_stack = create_component::<FrameStackView>(());
        content.add_named(frame_stack.widget(), "vm");

        let landing_page = create_component::<LandingPage>(());
        content.add_named(landing_page.widget(), "landing");

        window.add(&content);

        let s = header.stream();
        connect_stream!(
            s@AppHeaderEvent::Load,
            relm.stream(),
            AppEvent::LoadRequest
        );
        let s = frame_stack.stream();
        connect_stream!(
            s@FrameStackEvent::FrameSelected(i),
            relm.stream(),
            AppEvent::FrameSelected(i)
        );

        // let l = label.clone();
        // let (channel, sender) = Channel::new(move |string: String| {
        //     l.set_label(&string);
        // });
        // model.vm.watch(move |event| {
        //     if let ItemEvent::EventA(s) = event {
        //         sender.send(s).expect("send message");
        //     }
        // });

        window.show_all();

        AppWindow {
            model,
            window,
            header,
            content,
            frame_stack,
        }
    }
}

struct VmParams {
    main_class: ClassName,
    class_path: Vec<String>,
}

impl AppWindow {
    fn load_css() {
        let css_data = include_bytes!("style.css");
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
}

fn show_load_dialog(parent: &Window) -> Option<VmParams> {
    // let mut file = None;
    // let dialog = FileChooserDialog::new(
    //     Some("Select an MP3 audio file"),
    //     Some(parent),
    //     FileChooserAction::Open,
    // );
    // let mp3_filter = FileFilter::new();
    // mp3_filter.add_mime_type("audio/mp3");
    // mp3_filter.set_name("MP3 audio file");
    // dialog.add_filter(&mp3_filter);
    // let m3u_filter = FileFilter::new();
    // m3u_filter.add_mime_type("audio/x-mpegurl");
    // m3u_filter.set_name("M3U playlist file");
    // dialog.add_filter(&m3u_filter);
    // dialog.add_button("Cancel", RESPONSE_CANCEL);
    // dialog.add_button("Accept", RESPONSE_ACCEPT);
    // let result = dialog.run();
    // if result == RESPONSE_ACCEPT {
    //     file = dialog.get_filename();
    // }
    // dialog.destroy();
    // file
    unimplemented!()
}
