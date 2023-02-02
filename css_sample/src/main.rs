use gtk::glib;
use gtk::prelude::*;

use gtk::gdk::Display;
use gtk::{
    Application, ApplicationWindow, Builder, Box as Box_, Button, ComboBoxText, CssProvider, Entry,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION
};

fn main() {
    let application = Application::new(Some("com.github.css"), Default::default());
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });
    application.run();
}

fn build_ui(application: &Application) {

    // Define the GUI xml
    let ui_src = include_str!("gui.ui");
    let builder = Builder::from_string(ui_src);


    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    // Define button

    let button: Button = builder.object("button").expect("Couldn't get button");
    button.add_css_class("button1");

    // Define Entry

    let entry: Entry = builder.object("entry").expect("Couldn't get entry");
    entry.add_css_class("entry1");
    entry.set_text("Some text");

    // Define combo box (drop down box)
    let combo: ComboBoxText = builder.object("combo").expect("Couldn't get combo");
    combo.append_text("option 1");
    combo.append_text("option 2");
    combo.append_text("option 3");
    combo.set_active(Some(0));



    application.connect_activate(move |_| {
        window.show();
    });
}