mod box_main;

use gtk::{
    self,
    prelude::*,
    glib::{
        self
    },
    Application,
    ApplicationWindow,
};

use box_main::build_box_main;

const APP_ID: &str = "com.lalunainsky.gtk_01"; 

fn build_ui(
    app: &Application
) {
    let box_buttons_increments_decrements = build_box_main();

    let window = ApplicationWindow::builder()
                .application(app)
                .title("GTK 01 App")
                .default_width(500)
                .default_height(500)
                .opacity(0.95)
                .resizable(false)
                .child(&box_buttons_increments_decrements)
                .build();

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
            .application_id(APP_ID)
            .build();

    app.connect_activate(build_ui);

    app.run()
}