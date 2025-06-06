use std::process::Command;
use rand;

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let num  = 10;

    println!(
        "Hello, world! {num} plus one is {}!", add_one::add_one(num)
    );
}