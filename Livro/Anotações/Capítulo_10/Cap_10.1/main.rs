use std::process::Command;

fn clean_of_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_of_terminal();

    println!("hello, world!");
}