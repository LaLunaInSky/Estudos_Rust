#[derive(Debug)]
enum List {
    Cons (Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::{
    process::Command,
    cell::RefCell,
    rc::Rc,
};
use crate::List::{
    Cons,
    Nil,
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let mut x = 5;
    let _y = &mut x;

    //
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(
        Cons(
            Rc::clone(&value),
            Rc::new(Nil)
        )
    );

    let b = Cons(
        Rc::new(
            RefCell::new(3)
        ),
        Rc::clone(&a)
    );

    let c = Cons(
        Rc::new(
            RefCell::new(4)
        ),
        Rc::clone(&a)
    );

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}