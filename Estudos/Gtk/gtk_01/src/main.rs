use gtk::{prelude::*, Button};

use gtk::{
    glib,
    Application,
    ApplicationWindow
};

const APP_ID: &str = "com.lalunainsky.gtk_01"; 

fn build_button() -> Button {
    let button = Button::builder()
                .label("Press me!")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

    button.connect_clicked(
        |button| {
            button.set_label("Hello World!")
        }
    );

    return button;
}

fn build_ui(
    app: &Application
) {
    let button_01 = build_button();

    let window = ApplicationWindow::builder()
                .application(app)
                .title("GTK 01 App")
                .default_width(500)
                .default_height(500)
                .opacity(0.95)
                .resizable(false)
                .child(&button_01)
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