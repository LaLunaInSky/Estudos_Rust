mod box_main;
mod box_main_02;

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
use box_main_02::build_box_main_02;

const APP_ID: &str = "com.lalunainsky.gtk_01"; 

fn build_ui(
    app: &Application
) {
    // let box_buttons_increments_decrements = build_box_main();
    let box_main_02 = build_box_main_02();

    let window = ApplicationWindow::builder()
                .application(app)
                .title("GTK 01 App")
                .default_width(500)
                .default_height(500)
                .opacity(0.95)
                .resizable(false)
                // .child(&box_buttons_increments_decrements)
                .child(&box_main_02)
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