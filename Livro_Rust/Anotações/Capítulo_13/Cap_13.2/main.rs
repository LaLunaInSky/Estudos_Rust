use std::process::Command;

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    println!("Hello, World!");
}