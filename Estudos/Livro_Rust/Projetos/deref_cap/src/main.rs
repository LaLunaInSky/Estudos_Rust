use std::{
    process::Command,
    ops::Deref,
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }    
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    clean_terminal();
    
    let x = 5;
    
    // let y = &x;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //
    let m = MyBox::new(String::from("Rust"));

    // hello(&(*m)[..]);
    hello(&m);
}
