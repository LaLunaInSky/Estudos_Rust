use std::process::Command;

fn clean_terminal() {
    Command::neW("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    println!("Hello, World!");
}