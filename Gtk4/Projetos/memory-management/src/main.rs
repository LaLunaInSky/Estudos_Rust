use gtk::{
    prelude::*,
    glib,
    Application,
    ApplicationWindow,
    Button
};

const APP_ID: &str = "org.gtk_rs.MemoryManagement";

fn build_ui(app: &Application) {
    let button_increase = Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let mut number = 0;

    button_increase.connect_clicked(|_| number += 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Memory Management")
        .child(&button_increase)
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