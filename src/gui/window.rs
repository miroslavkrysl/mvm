use super::{
    frame_stack::{FrameStackMsg, FrameStackView},
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
};
use gdk::Screen;
use gtk::{Button, ContainerExt, CssProviderExt, Entry, GtkWindowExt, Inhibit, Label, Orientation, Stack, StackExt, StackSwitcher, StackSwitcherExt, StyleContext, WidgetExt, Window, WindowType, STYLE_PROVIDER_PRIORITY_APPLICATION, SpinnerExt};
use relm::{
    connect, connect_stream, create_component, init, interval, Channel, Component, Relm, Update,
    Widget,
};
use relm_derive::Msg;
use crate::vm::exec::vm::Vm;
use std::path::PathBuf;
use std::sync::Arc;
use crate::gui::locals::{LocalsView, LocalsMsg};
use crate::gui::operand_stack::{OperandStackView, OperandStackMsg};
use crate::gui::fields::{FieldsView, FieldsMsg, Viewed};
use crate::gui::classes::{ClassesView, ClassesMsg};
use crate::gui::instances::{InstancesView, InstancesMsg};


pub struct AppState {
    vm: Arc<Vm>
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
    locals: Component<InstancesView>
    // frame_stack: Component<FrameStackView>,
}

impl Update for AppWindow {
    type Model = AppState;
    type ModelParam = ();
    type Msg = AppEvent;

    fn model(_: &Relm<Self>, _: ()) -> AppState {
        let vm = Vm::new(vec!["./".into()]);
        AppState {
            vm: Arc::new(vm)
        }
    }

    fn update(&mut self, event: AppEvent) {
        match event {
            AppEvent::LoadRequest => {
                // let params = show_load_dialog(&self.window);
                // self.window.maximize();
                // self.content.set_visible_child_name("vm");
                // println!("Load request");

                // vm.set_update_callback(Some(Box::new(|vm| {
                //     println!("hello");
                // })));
                // let e = ended.clone();
                // vm.set_error_callback(Some(Box::new(move |vm, error| {
                //     println!("{:?}", error);
                //     e.store(true, Ordering::SeqCst);
                // })));
                // let e = ended.clone();
                // vm.set_end_callback(Some(Box::new(move |vm| {
                //     println!("end");
                //     e.store(true, Ordering::SeqCst);
                // })));
                //
                // vm.clone().start(class_name);

                // self.frame_stack.emit(FrameStackMsg::Push(
                //     ClassName::new("hello").unwrap(),
                //     MethodSig::new(
                //         ReturnDesc::Void,
                //         MethodName::new("hello").unwrap(),
                //         [TypeDesc::Int].iter().cloned().collect(),
                //     )
                //     .unwrap(),
                // ));
                self.model.vm.next();
                println!("next");
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

        let header = create_component::<AppHeaderView>(());
        window.set_titlebar(Some(header.widget()));

        let content = Stack::new();

        // let frame_stack = create_component::<FrameStackView>(());
        // content.add_nam

        let locals = create_component::<InstancesView>(());
        content.add_named(locals.widget(), "locals");

        // let landing_page = create_component::<LandingPage>(());
        // content.add_named(landing_page.widget(), "landing");

        window.add(&content);

        connect!(
            header@AppHeaderEvent::Load,
            relm,
            AppEvent::LoadRequest
        );

        window.show_all();

        let vm = model.vm.clone();
        let vm0 = vm.clone();
        let l = locals.clone();
        let (channel, sender) = Channel::new(move |_| {
            let frames = vm0.frames().unwrap();
            // let locals = frames.last().and_then(|frame| {
            //     Some(frame.locals().values())
            // }).or(Some(Vec::new())).unwrap();

            // let values = frames.last().and_then(|frame| {
            //     Some(frame.stack().values())
            // }).or(Some(Vec::new())).unwrap();
            let instances = vm0.instances();
            l.emit(InstancesMsg::Update(instances));
        });

        vm.set_update_callback(Some(Box::new(move |vm: &Vm| {
            sender.send(());
        })));
        vm.set_error_callback(Some(Box::new(move |vm: &Vm, error| {
            println!("{}", error);
        })));

        vm.clone().start(ClassName::new("geometry.shape.Circle").unwrap());

        AppWindow {
            model,
            window,
            header,
            content,
            locals
            // frame_stack,
        }
    }
}

struct VmParams {
    main_class: ClassName,
    class_path: Vec<String>,
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
