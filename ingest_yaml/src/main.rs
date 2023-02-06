use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button};
//use gtk:: glib;
use std::process::Command;


#[derive(Serialize, Deserialize, Debug)]
struct IngestCom {
    function_1 : String,
    command_1 : String,
    function_2 : String,
    command_2 : String,
    function_3 : String,
    command_3 : String,
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {

    let mut file = File::open("ingest_config.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let inc: IngestCom = serde_yaml::from_str(&contents).unwrap();

    // Define the GUI xml
    let ui_src = include_str!("gui.ui");
    let builder = Builder::from_string(ui_src);

    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    // Define button 1
    let btn_function1: Button = builder
        .object(&inc.function_1.to_string())
        .expect("Couldn't get button");

    // Action of the button
    btn_function1.connect_clicked(move |_|{
        term_command(inc.command_1.to_string());
    });

    // Define button 2
    let btn_function2: Button = builder
        .object(&inc.function_2.to_string())
        .expect("Couldn't get button");

    // Action of the button
    btn_function2.connect_clicked(move |_|{
        term_command(inc.command_2.to_string());
    });

    // Define button 3
    let btn_function3: Button = builder
        .object(&inc.function_3.to_string())
        .expect("Couldn't get button");

    // Action of the button
    btn_function3.connect_clicked(move |_|{
        term_command(inc.command_3.to_string());
    });

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
