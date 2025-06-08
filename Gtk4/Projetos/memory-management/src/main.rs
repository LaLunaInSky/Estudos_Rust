use gtk::{
    prelude::*,
    glib,
    Application,
    ApplicationWindow
};

const APP_ID: &str = "org.gtk_rs.MemoryManagement";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Memory Management")
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