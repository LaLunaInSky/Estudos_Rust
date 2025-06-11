use std::process::Command;

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let mut x = 5;
    let y = &mut x;
}