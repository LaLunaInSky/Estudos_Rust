mod counter;
mod message;

use crate::{
    counter::Counter,
    message::Message
};

#[test]
fn it_counts_properly() {
    let mut counter = Counter::default();

    counter.update(
        Message::Increment
    );

    counter.update(
        Message::Increment
    );

    counter.update(
        Message::Decrement
    );

    assert_eq!(
        counter.get_value(),
        1
    );
}