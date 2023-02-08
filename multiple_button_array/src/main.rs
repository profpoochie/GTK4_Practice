use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct ButtonList {
    name: String,
    command: String,
}

#[derive(Deserialize, Debug)]
struct Buttons {
    buttons: Vec<ButtonList>,
}

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &gtk::Application) {

    let mut file = File::open("ingest_config.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let buttons: Buttons = serde_yaml::from_str(&contents).unwrap();

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Generate Buttons")
        .default_width(300)
        .default_height(300)
        .build();

    let container = Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .spacing(24)
        .build();

    // setting buttons based on YAML config file.
    for button in buttons.buttons {
        let buttons = Button::with_label(&button.name);
        let actions = button.command.clone();
        buttons.connect_clicked(move |_|{
            term_command(actions.to_string());
        });
        container.append(&buttons);
    }


    window.set_child(Some(&container));
    window.show();
}

// Terminal command
fn term_command(input_string:String) {
    let input_vec: Vec<&str> = input_string.split(" ").collect();
    let command = input_vec[0];
    let args = &input_vec[1..];
    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

