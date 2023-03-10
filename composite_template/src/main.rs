mod ex_application_window;
pub mod ex_menu_button;

use ex_application_window::ExApplicationWindow;
use gtk::glib;
use std::process::ExitCode;
use gtk::prelude::*;

fn main() -> ExitCode {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.composite_template"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = ExApplicationWindow::new(app);
        win.show();
    });
    application.run()
}
