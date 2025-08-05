use gtk::{
    self,
    prelude::*,
    Align,
    Switch,
    Orientation
};

pub fn build_box_main_02() -> gtk::Box {
    let box_frame = gtk::Box::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .valign(Align::Center)
                .halign(Align::Center)
                .spacing(12)
                .orientation(Orientation::Vertical)
                .build();

    let switch_01 = Switch::new();
    let switch_02 = Switch::new();

    switch_01.bind_property("active", &switch_02, "active")
             .bidirectional()
             .build();

    box_frame.append(&switch_01);
    box_frame.append(&switch_02);

    return box_frame;
}