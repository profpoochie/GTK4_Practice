
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, Entry};
use gtk:: glib;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
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

    // Define button
    let button: Button = builder.object("button").expect("Couldn't get button");

    // Define text box or Entry
    let entry: Entry = builder.object("entry").expect("Couldn't get Entry");

    // Action of the button
    button.connect_clicked(clone!(@weak entry => move |_btn| {
        let input_string = entry.text().to_string();
        term_command(input_string);
    }));

    window.show();
}

// Terminal command
fn term_command(command:String) {
    let cmd = format!("gnome-terminal -e 'bash -c \"{}; exec $SHELL\"'", command.replace("\"", "\\\""));

    let status = Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn new terminal window to execute command")
        .wait()
        .expect("Failed to wait for command to complete");

    if status.success() {
        println!("Command executed successfully");
    } else {
        println!("Command failed with exit code {:?}", status.code());
    }
}