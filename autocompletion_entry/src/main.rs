use gtk::prelude::*;

use glib::Type;
use gtk::{Application, ApplicationWindow, Builder, Entry, EntryCompletion, Label, ListStore};
use gtk:: glib;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.clipboard"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &Application) {
    // Define the GUI xml
    let ui_src = include_str!("autocomplete_gui.ui");
    let builder = Builder::from_string(ui_src);

    // Define window
    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    // Define label
    let win_title: Label = builder.object("win_title").expect("Couldn't get label");
    win_title.set_markup("<big>Name a Programming Language you know.</big>");

    // Define text box or Entry
    let input_field: Entry = builder.object("input_field").expect("Couldn't get Entry");

    // Create an EntryCompletion widget
    let completion_countries = EntryCompletion::new();
    // Use the first (and only) column available to set the autocompletion text
    completion_countries.set_text_column(0);
    // how many keystrokes to wait before attempting to autocomplete?
    completion_countries.set_minimum_key_length(1);
    // whether the completions should be presented in a popup window
    completion_countries.set_popup_completion(true);

    let ls = create_list_model();
    completion_countries.set_model(Some(&ls));

    input_field.set_completion(Some(&completion_countries));

    window.show();
}

struct Data {
    description: String,
}

fn create_list_model() -> ListStore {
    let col_types: [Type; 1] = [Type::STRING];

    let data: [Data; 6] = [
        Data {
            description: "Python".to_string(),
        },
        Data {
            description: "Java".to_string(),
        },
        Data {
            description: "C and C++".to_string(),
        },
        Data {
            description: "C#".to_string(),
        },
        Data {
            description: "Rust".to_string(),
        },
        Data {
            description: "Ruby".to_string(),
        },
    ];
    let store = ListStore::new(&col_types);
    for d in data.iter() {
        store.set(&store.append(), &[(0, &d.description)]);
    }
    store
}