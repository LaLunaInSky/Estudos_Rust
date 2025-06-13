use std::{
    process::Command,
    time::Duration,
    thread
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let handle_01 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle_01.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle_01.join().unwrap();

    //
    println!();

    let v = vec![1, 2, 3];

    let hanlde_02 = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    hanlde_02.join().unwrap();
}