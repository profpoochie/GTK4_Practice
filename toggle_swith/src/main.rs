//use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Switch, Label};
//use gtk:: glib;
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
    let ui_src = include_str!("toggle_switch.ui");
    let builder = Builder::from_string(ui_src);

    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));


    let switch: Switch = builder.object("switch").expect("Couldn't get checkbutton");
    let label_switch: Label = builder.object("label_switch").expect("Couldn't get label");

    // Toggling the switch

    switch.connect_state_notify(move |switch|{
        if switch.is_active() {
            label_switch.set_text("Switch is now ON.");
        } else {
            label_switch.set_text("Switch is now OFF.");
        }
    });


    window.show();

}

