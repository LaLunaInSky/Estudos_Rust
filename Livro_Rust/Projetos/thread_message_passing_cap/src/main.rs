use std::{
    process::Command,
    sync::mpsc,
    thread
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    
    println!("Got: {received}");
}