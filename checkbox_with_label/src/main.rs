use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, Entry, CheckButton, Label};
use gtk:: glib;
//use std::process::Command;


fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}


fn build_ui(application: &Application) {
    // Define the GUI xml
    let ui_src = include_str!("checkbutton.ui");
    let builder = Builder::from_string(ui_src);

    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));


    let checkbutton: CheckButton = builder.object("checkbutton").expect("Couldn't get checkbutton");
    let label_checkbutton: Label = builder.object("label_checkbutton").expect("Couldn't get label");

    checkbutton.connect_toggled(move |checkbutton| {
        if checkbutton.is_active() {
            label_checkbutton.set_text("Button is now active.");
        } else {
            label_checkbutton.set_text("Button is now inactive.");
        }
    });


    window.show();

}

