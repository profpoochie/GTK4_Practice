use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button};

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Generate Buttons Example")
        .default_width(300)
        .default_height(300)
        .build();
    let names = vec!["Button 1", "Button 2", "Button 3", "Button 4"];
    let buttons: Vec<Button> = names
        .into_iter()
        .map(|name| {
            let button = Button::with_label(name);
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

