use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Menu, MenuBar, MenuItem};

fn main() {
    let app = Application::builder()
        .application_id("com.example.gtk-rs-menu")
        .build();

    app.connect_activate(|app| {
        // Create the application window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(240)
            .title("GTK4 Menu Example")
            .build();

        // Create the menu bar
        let menu_bar = MenuBar::new();
        window.set_titlebar(Some(&menu_bar));

        // Create the file menu
        let file_menu = Menu::new();
        let file_item = MenuItem::with_label("File");
        file_item.set_submenu(Some(&file_menu));
        menu_bar.append(&file_item);

        // Add a Quit menu item to the file menu
        let quit_item = MenuItem::with_label("Quit");
        quit_item.connect_activate(|_| {
            gtk::main_quit();
        });
        file_menu.append(&quit_item);

        // Show the window and run the GTK main loop
        window.show_all();
    });

    app.run();
}
