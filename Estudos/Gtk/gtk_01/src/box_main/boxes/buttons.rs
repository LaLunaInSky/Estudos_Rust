use gtk::{
    self,
    prelude::*,
    Button
};

pub fn build_button_increment() -> gtk::Button {
    let button = gtk::Button::builder()
                .label("Increment")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

    return button;
}

pub fn build_button_decrement() -> gtk::Button {
    let button = gtk::Button::builder()
                .label("Decrement")
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

    return button;
}

pub fn build_button_hello_world() -> gtk::Button {
    let button = gtk::Button::builder()
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