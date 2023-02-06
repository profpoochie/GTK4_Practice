use chrono::Local;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Label};

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.clock"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    // Define the GUI xml
    let ui_src = include_str!("text_gui.ui");
    let builder = Builder::from_string(ui_src);

    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));


    let time = current_time();
    let label: Label = builder.object("label").expect("Couldn't get label");
    label.set_text(&time);

    window.set_child(Some(&label));

    window.show();

    // we are using a closure to capture the label (else we could also use a normal function)
    let tick = move || {
        let time = current_time();
        label.set_text(&time);
        // we could return gtk::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}

fn current_time() -> String {
    format!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
}