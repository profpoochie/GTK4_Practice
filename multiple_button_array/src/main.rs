use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Grid};
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

    let container = Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    let mut counter=0;
    let mut column=0;
    let mut row=0;
    window.set_child(Some(&container));
    // setting buttons based on YAML config file.
    for button in buttons.buttons {

        row = counter / 6;
        column = counter % 6;

        println!("counter:{}, column:{}, row:{}", counter,column,row);
        let buttons = Button::with_label(&button.name);
        let actions = button.command.clone();
        buttons.connect_clicked(move |_|{
            term_command(actions.to_string());
        });
        container.attach(&buttons,column,row,1,1);
        counter = counter+1;
    }



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

