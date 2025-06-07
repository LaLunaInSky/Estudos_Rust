use std::process::Command;
use crate::List::{
    Cons, Nil
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    clean_terminal();

    let b_01 = Box::new(5);

    println!("b_01 = {b_01}");

    //
    println!();

    let list_01 = Cons(1, Cons(2, Cons(3, Nil)));
}