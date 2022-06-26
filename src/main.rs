use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk-rs.HelloWorld3";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();    // Create a new application
    app.connect_activate(build_ui);     // Connect to "activate" signal of `app`
    app.run();      // Run the application
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

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App") // Window title
        .child(&button)
        .build();

    window.present();       // Present window
    window.show_all();      // Will not work with no show_all
}
