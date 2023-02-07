use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};
//use core::result::IntoIter;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
struct IngestCom {
    buttons: Vec<String>,
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

    let inc: IngestCom = serde_yaml::from_str(&contents).unwrap();

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Generate Buttons Example")
        .default_width(300)
        .default_height(300)
        .build();
    let names = inc.buttons;

    // setting buttons based on YAML config file.
    let buttons: Vec<Button> = names
        .into_iter()
        .map(|name| {
            let button = Button::with_label(&name);
            button.connect_clicked(move |_| {
                println!("Button clicked");
            });
            button
        })
        .collect();

    let container = Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        .build();
    for button in buttons {
        container.append(&button);
    }
    window.set_child(Some(&container));
    // window.show_all();
    // application.run(&[]);
    window.show();
}

