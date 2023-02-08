use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Button {
    name: String,
    command: String,
}

#[derive(Deserialize, Debug)]
struct Buttons {
    buttons: Vec<Button>,
}

fn main() {
    let mut file = File::open("dev_ui_config.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let buttons: Buttons = serde_yaml::from_str(&contents).unwrap();

    for button in buttons.buttons {
        println!("Name: {}", button.name);
        println!("Command: {}", button.command);
        println!("");
    }
}
