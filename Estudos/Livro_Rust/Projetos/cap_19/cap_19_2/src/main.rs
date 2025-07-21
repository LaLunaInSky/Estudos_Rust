use std::{
    process::Command
};

fn clean_terminal_at_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    // clean_terminal_at_linux();

    let some_option_value: Option<i32> = None;

    // let Some(x) = some_option_value;
    let Some(x) = some_option_value else {
        return;
    };

    let x = 5 else {
        return;
    };
}