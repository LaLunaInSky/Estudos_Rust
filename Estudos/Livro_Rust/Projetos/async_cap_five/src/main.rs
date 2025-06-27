use std::process::Command;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();
    
    println!("Hello, world!");
}