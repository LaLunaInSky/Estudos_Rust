use gtk::{
    prelude::*,
    Orientation
};

mod boxes;

use boxes::{
    build_box_custom_button,
    build_box_buttons_increment_and_decrement_02,
    build_box_buttons_increment_and_decrement_01,
    build_box_button_hello_world
};

pub fn build_box_main() -> gtk::Box {
    let box_frame_custom_button = build_box_custom_button();
    let box_frame_02 = build_box_buttons_increment_and_decrement_02();
    let box_frame_01 = build_box_buttons_increment_and_decrement_01();
    let box_frame_hello_world = build_box_button_hello_world();

    let box_frame_main = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    box_frame_main.append(&box_frame_hello_world);
    box_frame_main.append(&box_frame_01);
    box_frame_main.append(&box_frame_02);
    box_frame_main.append(&box_frame_custom_button);

    return box_frame_main;
}