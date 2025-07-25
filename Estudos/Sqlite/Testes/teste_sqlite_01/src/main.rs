use sqlite::*;

fn main() {
    let connection = sqlite::open("sharks.db").unwrap();

    let cursor = connection::cursor();
}