use gtk::{
    self,
    glib,
    prelude::*,
    subclass::prelude::*
};

use glib::Properties;

use std::cell::Cell;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    number: Cell<i32>,
}

#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "Gtk01CustomButtom";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

#[glib::derived_properties]
impl ObjectImpl for CustomButton {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.bind_property("number", obj.as_ref(), "label")
           .sync_create()
           .build();
    }
}

impl WidgetImpl for CustomButton {}

impl ButtonImpl for CustomButton {
    fn clicked(&self) {
        let increment_number = self.obj().number() + 1;
        
        self.obj().set_number(increment_number);
    }
}