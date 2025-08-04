use gtk::{
    prelude::*,
    Orientation,
    glib::{
        self,
        clone
    }
};

use std::{
    cell::Cell,
    rc::Rc
};

mod buttons;

use buttons::{
    build_button_increment,
    build_button_decrement,
    build_button_hello_world
};

pub fn build_box_button_hello_world() -> gtk::Box {
    let box_frame = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    let button_hello_world = build_button_hello_world();
    let label = gtk::Label::builder()
            .label("Box Hello World")
            .margin_top(10)
            .build();

    box_frame.append(&label);
    box_frame.append(&button_hello_world);

    return box_frame;
}

pub fn build_box_buttons_increment_and_decrement_01() -> gtk::Box {
    let button_increment = build_button_increment();
    let button_decrement = build_button_decrement();

    let label = gtk::Label::builder()
            .label("Box 01")
            .margin_top(5)
            .build();

    let box_frame = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    box_frame.append(&label);
    box_frame.append(&button_increment);
    box_frame.append(&button_decrement);

    let number = Rc::new(Cell::new(0));

    /* add functions buttons */
    button_increment.connect_clicked(clone!(
        #[strong]
        number,
        move |_| {
            number.set(number.get() + 1);
            
            println!(
                "Increment! Number is {}",
                number.get()
            );
        }
    ));

    button_decrement.connect_clicked(
        move |_| {
            number.set(number.get() - 1);

            println!(
                "Decrement! Number is {}",
                number.get()
            );
        }
    );

    return box_frame;
}

pub fn build_box_buttons_increment_and_decrement_02() -> gtk::Box {
    let button_increment = build_button_increment();
    let button_decrement = build_button_decrement();

    let label = gtk::Label::builder()
        .label("Box 02")
        .margin_top(12)
        .build();

    let box_frame = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    box_frame.append(&label);
    box_frame.append(&button_increment);
    box_frame.append(&button_decrement);

    let number = Rc::new(Cell::new(0));

    button_increment.connect_clicked(clone!(
        #[weak]
        number,
        #[strong]
        button_decrement,
        move |_| {
            number.set(number.get() + 1);

            button_decrement.set_label(&number.get().to_string());
        }
    ));

    button_decrement.connect_clicked(clone!(
        #[strong]
        button_increment,
        move |_| {
            number.set(number.get() - 1);

            button_increment.set_label(&number.get().to_string());
        }
    ));

    return box_frame;
}