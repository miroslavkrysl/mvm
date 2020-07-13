use gui::AppWindow;
use relm::Widget;

pub mod gui;
pub mod vm;

fn main() {
    // let application = Application::new(
    //     Some("cz.mkrysl.mvm"),
    //     Default::default(),
    // );

    // match application {
    //     Ok(application) => {
    //         application.connect_startup(|app| {
    //             let window = MainWindow::new();
    //             app.add_window(window.window());

    //         });

    //         application.run(&args().collect::<Vec<_>>());
    //     }
    //     Err(_) => {}
    // }

    // let vm = VirtualMachine::new();
    AppWindow::run(()).unwrap();
}
