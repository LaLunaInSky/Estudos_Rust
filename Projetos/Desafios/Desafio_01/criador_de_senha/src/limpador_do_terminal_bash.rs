use std::{
    process::Command
};

pub fn limpar_o_terminal_bash() {
    Command::new("clear").status().unwrap();
}