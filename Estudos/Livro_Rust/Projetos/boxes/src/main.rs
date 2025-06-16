use std::process::Command;
use crate::List::{
    Cons, Nil
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    clean_terminal();

    let b_01 = Box::new(5);

    println!("b_01 = {b_01}");

    //
    println!();

    let list_01 = Box::new(1, Box::new(2, Box::new(3, Box::new(Nil))));
}