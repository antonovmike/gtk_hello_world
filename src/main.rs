use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label};

fn main() {
    let app = Application::builder().build(); // Create a new application
    app.connect_activate(build_ui); // Connect to "activate" signal of `app`
    app.run(); // Run the application
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // let label = Label::new(Some("Hello world"));
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App") // Window title
        .child(&button) // Button inside the window
        // .child(&label) // Text inside the window
        .build();
    // window.add(&label);
    // window.set_default_size(320, 140); // Sets windows's custom size
    window.present(); // Present window. Sets windows's default size
    window.show_all(); // Will not work with no show_all
}

// https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/struct.Builder.html
