use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, Button};
use std::process::Command;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let ui_src = include_str!("gtk_builder.ui");
    let builder = Builder::from_string(ui_src);

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let button1: Button = builder.object("button1").expect("Couldn't get button");
    let button2: Button = builder.object("button2").expect("Couldn't get button");
    let button3: Button = builder.object("button3").expect("Couldn't get button");
    let button4: Button = builder.object("button4").expect("Couldn't get button");
    let button5: Button = builder.object("button5").expect("Couldn't get button");
    let button6: Button = builder.object("button6").expect("Couldn't get button");
    let button7: Button = builder.object("button7").expect("Couldn't get button");
    let button8: Button = builder.object("button8").expect("Couldn't get button");
    let button9: Button = builder.object("button9").expect("Couldn't get button");

    button1.connect_clicked(move |_| {
        term_command();
    });
    button2.connect_clicked(move |_| {
        term_command();
    });
    button3.connect_clicked(move |_| {
        term_command();
    });
    button4.connect_clicked(move |_| {
        term_command();
    });
    button5.connect_clicked(move |_| {
        term_command();
    });
    button6.connect_clicked(move |_| {
        term_command();
    });
    button7.connect_clicked(move |_| {
        term_command();
    });
    button8.connect_clicked(move |_| {
        term_command();
    });
    button9.connect_clicked(move |_| {
        term_command();
    });

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