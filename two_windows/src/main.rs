use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Window};

fn main() {
    let app = Application::builder()
        .application_id("com.example.myapp")
        .build();

    app.connect_activate(|app| {
        let main_window = ApplicationWindow::builder()
            .application(app)
            .title("Main Window")
            .default_width(320)
            .default_height(200)
            //.position(WindowPosition::Center)
            .build();
        let spawn_window_button = Button::builder()
            .label("Spawn New Window")
            .margin_top(50)
            .margin_bottom(50)
            .margin_start(50)
            .margin_end(50)
            .build();
        spawn_window_button.connect_clicked(move |_| {
            let new_window = Window::builder()
                .title("New Window")
                .default_width(320)
                .default_height(200)
                .build();

            let latest_version_button = Button::builder()
                .label("Latest Version")
                .margin_top(50)
                .margin_bottom(50)
                .margin_start(50)
                .margin_end(50)
                .build();

            new_window.set_child(Some(&latest_version_button));
            new_window.show();
            latest_version_button.connect_clicked(move |_| {
                new_window.close();
            });



        });

        main_window.set_child(Some(&spawn_window_button));
        main_window.show();
    });

    app.run();
}
