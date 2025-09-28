use crate::message::Message;

use iced::widget::{
    button,
    text,
    column,
    Column
};

#[derive(Default)]
pub struct Counter {
    value: i64
}

impl Counter {
    pub fn update(
        &mut self,
        message: Message
    ) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    pub fn get_value(
        &self
    ) -> i64 {
        return self.value;
    }

    pub fn view(
        &self
    ) -> Column<'_, Message> {
        let interface = column![
            button(
                "+"
            ).on_press(
                Message::Increment
            ),
            text(
                self.get_value()
            ),
            button(
                "-"
            ).on_press(
                Message::Decrement
            )
        ];

        return interface;
    }
}