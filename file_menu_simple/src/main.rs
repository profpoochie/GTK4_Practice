use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Menu, MenuBar, MenuButton, MenuItem, HeaderBar, Box, Label};

fn main() {
    let app = Application::new(Some("com.example.gtk4app"), Default::default());

    app.connect_activate(|app| {
        // Create a window with a title
        let window = ApplicationWindow::new(app);
        window.set_title("GTK4 Rust App");

        // Create a header bar with a label
        let header_bar = HeaderBar::new();
        header_bar.set_title(Some("GTK4 Rust App"));
        window.set_titlebar(Some(&header_bar));

        // Create a menu bar
        let menu_bar = MenuBar::new();
        let file_menu = MenuItem::with_label("File");

        let edit_menu = MenuItem::with_label("Edit");
        let view_menu = MenuItem::with_label("View");

        let menu_button = MenuButton::new();
        let menu_button_box = Box::new(gtk::Orientation::Horizontal, 0);
        menu_button_box.pack_start(&Label::new(None), true, true, 0);
        menu_button.set_child(Some(&menu_button_box));

        let file_submenu = gio::Menu::new();
        file_menu.set_submenu(Some(&file_submenu));
        file_submenu.append(Some("New"), Some("app.new"));
        file_submenu.append(Some("Open"), Some("app.open"));
        file_submenu.append(Some("Save"), Some("app.save"));
        file_submenu.append(Some("Quit"), Some("app.quit"));

        let edit_submenu = gio::Menu::new();
        edit_submenu.append(Some("Undo"), Some("app.undo"));
        edit_submenu.append(Some("Redo"), Some("app.redo"));
        edit_submenu.append(Some("Cut"), Some("app.cut"));
        edit_submenu.append(Some("Copy"), Some("app.copy"));
        edit_submenu.append(Some("Paste"), Some("app.paste"));
        edit_submenu.append(Some("Select All"), Some("app.select-all"));
        edit_menu.set_submenu(Some(&edit_submenu));

        let view_submenu = gio::Menu::new();
        view_submenu.append(Some("Zoom In"), Some("app.zoom-in"));
        view_submenu.append(Some("Zoom Out"), Some("app.zoom-out"));
        view_menu.set_submenu(Some(&view_submenu));

        menu_bar.append(&file_menu);
        menu_bar.append(&edit_menu);
        menu_bar.append(&view_menu);

        menu_button.set_menu_model(Some(&file_submenu));

        // Add the menu button to the header bar
        header_bar.pack_start(&menu_button);

        window.set_default_size(640, 480);
        window.show_all();
    });

    app.run();
}
