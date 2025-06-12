use std::{
    process::Command,
    cell::RefCell,
    rc::{
        Rc,
        Weak
    }
};

use crate::List::{
    Cons,
    Nil
};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let a = Rc::new(
        Cons(
            5, RefCell::new(
                Rc::new(Nil)
            )
        )
    );

    println!(
        "a initial rc count = {}", Rc::strong_count(&a)
    );

    println!(
        "a next item = {:?}", a.tail()
    );

    let b = Rc::new(
        Cons(
            10, RefCell::new(
                Rc::clone(&a)
            )
        )
    );

    println!(
        "\na rc count after b creation = {}", Rc::strong_count(&a)
    );

    println!(
        "b initial rc count = {}", Rc::strong_count(&b)
    );

    println!(
        "b next item = {:?}", b.tail()
    );

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!(
        "\nb rc count after changing a = {}", Rc::strong_count(&b)
    );

    println!(
        "a rc count after changing a = {}", Rc::strong_count(&a)
    );

    // println!(
    //     "a next item = {:?}", a.tail()
    // );

    //
    println!();

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent =  {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}