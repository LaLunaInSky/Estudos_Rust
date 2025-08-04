use gtk::{
    prelude::*
};

use gtk::{
    self,
    glib::{
        self,
        clone
    },
    Application,
    ApplicationWindow,
    Orientation
};

use std::{
    cell::Cell,
    rc::Rc
};

mod frame_main;

use frame_main::{
    build_button_increment,
    build_button_decrement
};

const APP_ID: &str = "com.lalunainsky.gtk_01"; 

fn build_box_buttons_increments_and_decrements() -> gtk::Box {
    let box_frame_01 = build_box_buttons_increment_and_decrement_01();
    let box_frame_02 = build_box_buttons_increment_and_decrement_02();

    let box_frame_main = gtk::Box::builder()
                .orientation(Orientation::Vertical)
                .build();

    box_frame_main.append(&box_frame_01);
    box_frame_main.append(&box_frame_02);

    return box_frame_main;
}

fn build_box_buttons_increment_and_decrement_01() -> gtk::Box {
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

fn build_box_buttons_increment_and_decrement_02() -> gtk::Box {
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

fn build_button_hello_world() -> gtk::Button {
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

fn build_ui(
    app: &Application
) {
    // let button_hello_world = build_button_hello_world();
    
    let box_buttons_increments_decrements = build_box_buttons_increments_and_decrements();

    let window = ApplicationWindow::builder()
                .application(app)
                .title("GTK 01 App")
                .default_width(500)
                .default_height(500)
                .opacity(0.95)
                .resizable(false)
                // .child(&button_hello_world)
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