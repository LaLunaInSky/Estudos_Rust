mod custom_button;

use custom_button::{
    CustomButton
};

use gtk::{
    self,
    prelude::*,
    glib
};

use glib::closure_local;

pub fn build_button_hello_world_closure_reactivity() -> gtk::Button {
    let button = gtk::Button::builder()
            .label("Press me!")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();

    button.connect_closure(
        "clicked", 
        false, 
        closure_local!(move |button: gtk::Button| {
            button.set_label("Hello, World!");
        }),
    );

    return button;
}

pub fn build_a_custom_button() -> CustomButton {
    let button = CustomButton::new();

    button.set_margin_top(12);
    button.set_margin_bottom(12);
    button.set_margin_start(12);
    button.set_margin_end(12);

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