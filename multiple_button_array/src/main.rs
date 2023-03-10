use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::process::{Command, Stdio};

const ROW_LIMIT: i32 = 20;

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
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(12)
        .column_spacing(12)
        .build();

    window.set_child(Some(&container));
    // setting buttons based on YAML config file.
    for (index,button) in buttons.buttons.iter().enumerate() {

        let row = (index as i32) % ROW_LIMIT;
        let column = (index as i32) / ROW_LIMIT;

        let buttons = Button::with_label(&button.name);
        let actions = button.command.clone();
        buttons.connect_clicked(move |_|{
            execute_command(actions.to_string());
        });
        container.attach(&buttons,column,row,1,1);
    }
    window.show();
}

// Terminal command
fn execute_command(command: String) {
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

