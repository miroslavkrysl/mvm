use std::path::PathBuf;
use std::sync::Arc;
use std::boxed::Box as StdBox;

use gtk::{Button, Entry, GtkWindowExt, Inhibit, Label, Orientation, Paned, Stack, StackExt, StackSwitcher, StackSwitcherExt, StyleContext, WidgetExt, Window, WindowType, PanedExt, ButtonBox, Box, ButtonBoxExt, ButtonBoxStyle, BoxExt, StyleContextExt, ButtonExt, MessageDialog, DialogFlags, DialogExt, MessageType, ButtonsType};
use relm::{Channel, Component, connect, create_component, Relm, Update, Widget, Sender};
use relm_derive::Msg;

use crate::gui::classes::{ClassesMsg, ClassesView};
use crate::gui::fields::{FieldsMsg, FieldsView, Viewed};
use crate::gui::instances::{InstancesView, InstancesMsg};
use crate::gui::instructions::{InstructionsMsg, InstructionsView};
use crate::gui::locals::{LocalsMsg, LocalsView};
use crate::gui::operand_stack::{OperandStackMsg, OperandStackView};
use crate::vm::{
    class::{
        descriptor::{ParamsDesc, ReturnDesc, TypeDesc},
        name::{ClassName, MethodName},
        signature::MethodSig,
    },
};
use crate::vm::exec::error::ExecError;
use crate::vm::exec::vm::Vm;

use super::{
    frame_stack::{FrameStackMsg, FrameStackView},
    header::AppHeaderView,
    landing::LandingPage,
};
use gdk::prelude::WindowExtManual;
use gtk::prelude::Cast;
use crate::vm::class::class::Class;
use crate::vm::memory::frame::Frame as VmFrame;
use crate::vm::class::instance::Instance;


#[derive(Msg)]
pub enum VmMsg {
    Update,
    Ended,
    Error(ExecError),
    NextClicked,
    ReloadClicked,

    SelectFrame(usize, Arc<VmFrame>),
    SelectClass(Arc<Class>),
    SelectInstance(Instance),
}

enum VmChannelMsg {
    Update,
    Ended,
    Error(ExecError),
}

pub struct VmState {
    vm: Arc<Vm>,
    main_class: ClassName,
    path: Vec<PathBuf>,
    joined: bool,
}


pub struct VmView {
    model: VmState,
    root: Paned,
    relm: Relm<VmView>,
    vm_channel: (Channel<VmChannelMsg>, Sender<VmChannelMsg>),
    frame_stack: Component<FrameStackView>,
    locals: Component<LocalsView>,
    operand_stack: Component<OperandStackView>,
    instructions: Component<InstructionsView>,
    instances: Component<InstancesView>,
    classes: Component<ClassesView>,
    fields: Component<FieldsView>,
    next_button: Button,
    reload_button: Button,
}


impl Update for VmView {
    type Model = VmState;
    type ModelParam = (ClassName, Vec<PathBuf>);
    type Msg = VmMsg;

    fn model(_: &Relm<Self>, args: (ClassName, Vec<PathBuf>)) -> VmState {
        VmState {
            vm: Arc::new(Vm::new(args.1.clone())),
            main_class: args.0,
            path: args.1,
            joined: false
        }
    }

    fn update(&mut self, event: VmMsg) {
        match event {
            VmMsg::Update => {
                let frames = self.model.vm.frames().unwrap();
                let frame = frames.last().unwrap();
                let classes = self.model.vm.classes();
                let instances = self.model.vm.instances();
                let locals = frame.locals().values();
                let operands = frame.stack().values();
                let class = frame.class().clone();
                let method = frame.method().clone();
                let pc = frame.pc();

                self.frame_stack.emit(FrameStackMsg::Update(frames));
                self.frame_stack.emit(FrameStackMsg::SelectTopFrame);
                self.locals.emit(LocalsMsg::Update(locals));
                self.operand_stack.emit(OperandStackMsg::Update(operands));
                self.instructions.emit(InstructionsMsg::ChangeViewed(class, method));
                self.instructions.emit(InstructionsMsg::SelectInstruction(pc));
                self.instances.emit(InstancesMsg::Update(instances));
                self.classes.emit(ClassesMsg::Update(classes));
                self.fields.emit(FieldsMsg::Update);
            }
            VmMsg::Ended => {
                self.next_button.set_sensitive(false);
                self.model.joined = true;
                self.model.vm.join();
            }
            VmMsg::Error(error) => {
                self.next_button.set_sensitive(false);
                let window = self.root.get_toplevel().unwrap().downcast::<Window>().unwrap();
                let dialog = MessageDialog::new(
                    Some(&window),
                    DialogFlags::MODAL,
                    MessageType::Error,
                    ButtonsType::Ok,
                    &format!("Error: {}", error)
                );
                dialog.connect_response(|dialog, _| dialog.close());
                dialog.show_all();
            }
            VmMsg::NextClicked => {
                self.model.vm.next();
            }
            VmMsg::ReloadClicked => {
                if !self.model.joined {
                    self.model.vm.set_end_callback(None);
                    self.model.vm.thread().unwrap().cancel();
                    self.model.vm.join();
                }

                self.model.joined = false;
                self.model.vm = Arc::new(Vm::new(self.model.path.clone()));
                self.connect();
                self.next_button.set_sensitive(true);
            }
            VmMsg::SelectFrame(index, frame) => {
                self.locals.emit(LocalsMsg::Update(frame.locals().values()));
                self.operand_stack.emit(OperandStackMsg::Update(frame.stack().values()));
                self.instructions.emit(InstructionsMsg::ChangeViewed(frame.class().clone(), frame.method().clone()));
                self.instructions.emit(InstructionsMsg::SelectInstruction(frame.pc()));
            }
            VmMsg::SelectClass(class) => {
                self.instances.emit(InstancesMsg::Unselect);
                self.fields.emit(FieldsMsg::ChangeViewed(Viewed::Class(class)));
            }
            VmMsg::SelectInstance(instance) => {
                self.classes.emit(ClassesMsg::Unselect);
                self.fields.emit(FieldsMsg::ChangeViewed(Viewed::Instance(instance)));
            }
        }
    }
}


impl Widget for VmView {
    type Root = Paned;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let frame_stack = create_component::<FrameStackView>(());
        let locals = create_component::<LocalsView>(());
        let operand_stack = create_component::<OperandStackView>(());
        let instructions = create_component::<InstructionsView>(());
        let instances = create_component::<InstancesView>(());
        let classes = create_component::<ClassesView>(());
        let fields = create_component::<FieldsView>(());

        let center_box = Box::new(Orientation::Vertical, 0);
        let paned_left_horizontal = Paned::new(Orientation::Horizontal);
        let paned_left_vertical = Paned::new(Orientation::Vertical);
        let paned_center = Paned::new(Orientation::Horizontal);
        let paned_right_vertical = Paned::new(Orientation::Vertical);
        let paned_right_horizontal = Paned::new(Orientation::Horizontal);
        let root = Paned::new(Orientation::Horizontal);

        let reload_button = Button::with_label("Reload");
        reload_button.get_style_context().add_class("destructive-action");
        let next_button = Button::with_label("Next");
        next_button.get_style_context().add_class("suggested-action");
        let control = ButtonBox::new(Orientation::Horizontal);
        control.set_property_margin(10);
        control.set_layout(ButtonBoxStyle::Edge);
        control.pack_start(&reload_button, false, false, 0);
        control.pack_start(&next_button, false, false, 0);

        center_box.pack_start(instructions.widget(), true, true, 0);
        center_box.pack_start(&control, false, true, 0);
        paned_left_vertical.pack1(locals.widget(), false, false);
        paned_left_vertical.pack2(operand_stack.widget(), false, false);
        paned_left_horizontal.pack1(frame_stack.widget(), false, false);
        paned_left_horizontal.pack2(&paned_left_vertical, false, false);
        paned_right_horizontal.pack1(instances.widget(), false, false);
        paned_right_horizontal.pack2(classes.widget(), false, false);
        paned_right_vertical.pack1(&paned_right_horizontal, false, false);
        paned_right_vertical.pack2(fields.widget(), false, false);
        paned_center.pack1(&center_box, true, false);
        paned_center.pack2(&paned_right_vertical, false, false);
        root.pack1(&paned_left_horizontal, false, false);
        root.pack2(&paned_center, true, false);

        connect!(
            relm,
            next_button,
            connect_clicked(_),
            VmMsg::NextClicked
        );
        connect!(
            relm,
            reload_button,
            connect_clicked(_),
            VmMsg::ReloadClicked
        );

        connect!(
            frame_stack@FrameStackMsg::FrameActivated(ref index, ref frame),
            relm,
            VmMsg::SelectFrame(index.clone(), frame.clone())
        );
        connect!(
            instances@InstancesMsg::InstanceActivated(ref instance),
            relm,
            VmMsg::SelectInstance(instance.clone())
        );
        connect!(
            classes@ClassesMsg::ClassActivated(ref class),
            relm,
            VmMsg::SelectClass(class.clone())
        );

        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |msg| {
            match msg {
                VmChannelMsg::Update => stream.emit(VmMsg::Update),
                VmChannelMsg::Ended => stream.emit(VmMsg::Ended),
                VmChannelMsg::Error(error) => stream.emit(VmMsg::Error(error)),
            }
        });

        let view = VmView {
            model,
            root,
            relm: relm.clone(),
            frame_stack,
            locals,
            operand_stack,
            instructions,
            instances,
            classes,
            fields,
            next_button,
            reload_button,
            vm_channel: (channel, sender)
        };

        view.connect();

        view
    }
}

impl VmView {
    fn connect(&self) {
        let s = self.vm_channel.1.clone();
        self.model.vm.set_update_callback(Some(StdBox::new(move || {
            s.send(VmChannelMsg::Update).unwrap();
        })));
        let s = self.vm_channel.1.clone();
        self.model.vm.set_end_callback(Some(StdBox::new(move || {
            s.send(VmChannelMsg::Ended).unwrap();
        })));
        let s = self.vm_channel.1.clone();
        self.model.vm.set_error_callback(Some(StdBox::new(move |error| {
            s.send(VmChannelMsg::Error(error)).unwrap();
        })));

        self.fields.emit(FieldsMsg::ChangeViewed(Viewed::None));

        self.model.vm.clone().start(self.model.main_class.clone());
    }
}


// fn show_load_dialog(parent: &Window) -> Option<VmParams> {
//     // let mut file = None;
//     // let dialog = FileChooserDialog::new(
//     //     Some("Select an MP3 audio file"),
//     //     Some(parent),
//     //     FileChooserAction::Open,
//     // );
//     // let mp3_filter = FileFilter::new();
//     // mp3_filter.add_mime_type("audio/mp3");
//     // mp3_filter.set_name("MP3 audio file");
//     // dialog.add_filter(&mp3_filter);
//     // let m3u_filter = FileFilter::new();
//     // m3u_filter.add_mime_type("audio/x-mpegurl");
//     // m3u_filter.set_name("M3U playlist file");
//     // dialog.add_filter(&m3u_filter);
//     // dialog.add_button("Cancel", RESPONSE_CANCEL);
//     // dialog.add_button("Accept", RESPONSE_ACCEPT);
//     // let result = dialog.run();
//     // if result == RESPONSE_ACCEPT {
//     //     file = dialog.get_filename();
//     // }
//     // dialog.destroy();
//     // file
//     unimplemented!()
// }
