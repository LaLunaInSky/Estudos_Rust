use glib::clone;

use gtk::{
    prelude::*,
    glib,
};

fn on_activate(
    application: &gtk::Application
) {
    let window = gtk::ApplicationWindow::new(application);
    
    let button = gtk::Button::with_label("Hello World!");

    button.connect_clicked(
        clone!(@weak window => move |_| window.close())
    );

    window.set_child(Some(&button));

    window.present();
}

fn main() {
    let app = gtk::Application::builder()
            .application_id("com.lalunainsky.gtk_01")
            .build();

    app.connect_activate(on_activate);

    app.run();
}