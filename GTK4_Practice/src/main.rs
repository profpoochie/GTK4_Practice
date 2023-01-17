use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use std::process::Command;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.add_button_gui_gtk4"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn term_command() {
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);
}

fn build_ui(application: &gtk::Application) {
    // Create a new window, set its title and default size
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("Add Button GUI GTK4"));
    window.set_default_size(300, 300);

    // Here we construct the grid that is going contain our buttons.
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

    // Add the grid in the window
    window.set_child(Some(&grid));

    // Create the button 1 and put it into the grid at (0, 0)
    let button_1 = gtk::Button::with_label("Button 1");
    button_1.connect_clicked(move |_| term_command());

    grid.attach(&button_1, 0, 0, 1, 1);

    // Create the button 2 and put it into the grid at (1, 0)
    let button_2 = gtk::Button::with_label("Button 2");
    button_2.connect_clicked(move |_| term_command());

    grid.attach(&button_2, 1, 0, 1, 1);

    // Create the button 3 and put it into the grid at (2, 0)
    let button_3 = gtk::Button::with_label("Button 3");
    button_3.connect_clicked(move |_| term_command());

    grid.attach(&button_3, 2, 0, 1, 1);

    // Create the button 4 and put it into the grid at (0, 1)
    let button_4 = gtk::Button::with_label("Button 4");
    button_4.connect_clicked(move |_| term_command());

    grid.attach(&button_4, 0, 1, 1, 1);

    // Create the button 5 and put it into the grid at (1, 1)
    let button_5 = gtk::Button::with_label("Button 5");
    button_5.connect_clicked(move |_| term_command());

    grid.attach(&button_5, 1, 1, 1, 1);

    // Create the button 6 and put it into the grid at (2, 1)
    let button_6 = gtk::Button::with_label("Button 6");
    button_6.connect_clicked(move |_| term_command());

    grid.attach(&button_6, 2, 1, 1, 1);

    // Create the button 7 and put it into the grid at (0, 2)
    let button_7 = gtk::Button::with_label("Button 7");
    button_7.connect_clicked(move |_| term_command());

    grid.attach(&button_7, 0, 2, 1, 1);

    // Create the button 8 and put it into the grid at (1, 2)
    let button_8 = gtk::Button::with_label("Button 8");
    button_8.connect_clicked(move |_| term_command());

    grid.attach(&button_8, 1, 2, 1, 1);

    // Create the button 9 and put it into the grid at (2, 2)
    let button_9 = gtk::Button::with_label("Button 9");
    button_9.connect_clicked(move |_| term_command());

    grid.attach(&button_9, 2, 2, 1, 1);


    // Create the quit button and put it into the grid at bottom
    let quit_button = gtk::Button::with_label("Quit");
    quit_button.connect_clicked(clone!(@weak window => move |_| window.destroy()));

    grid.attach(&quit_button, 0, 3, 2, 1);

    window.show();
}