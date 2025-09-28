mod counter;
pub mod message;

use message::Message;
use crate::counter::Counter;

fn main() {
    iced::run(
        "A cool counter",
        Counter::update,
        Counter::view
    );
}