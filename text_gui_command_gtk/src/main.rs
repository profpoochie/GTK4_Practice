
use glib::clone;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button, Entry};
use gtk::{gdk, glib};
use std::process::Command;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &Application) {
    let ui_src = include_str!("text_gui.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let button: Button = builder.object("button").expect("Couldn't get button");

    let entry: Entry = builder.object("entry").expect("Couldn't get button");



    /*button.connect_clicked(move |_| {
        term_command();
    });*/

    button.connect_clicked(clone!(@weak entry => move |_btn| {
        let input_string = entry.text();
        let input_vec: Vec<&str> = input_string.split(" ").collect();
        let command = input_vec[0];
        let args = &input_vec[1..];
        let output = Command::new(command)
                    .args(args)
                    .output()
                    .expect("Failed to run command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
        //clipboard.set_text(&text);
    }));



    window.show();
}

fn term_command() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}