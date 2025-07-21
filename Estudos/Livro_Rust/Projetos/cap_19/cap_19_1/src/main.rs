use std::{
    thread::{
        sleep,
        spawn
    },
    time::Duration,
    process::Command,
    sync::mpsc::channel
};

use trpl::{
    run
};

fn clean_terminal_at_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_at_linux();

    // Using if let with else if

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!(
            "Using your favorite color, {}, as the background",
            color
        );
    } else if is_tuesday {
        println!(
            "Tuesday is green day!"
        );
    } else if let Ok(age) = age {
        if age > 30 {
            println!(
                "Using purple as the background color"
            );
        } else {
            println!(
                "Using orange as the background color"
            );
        }
    } else {
        println!(
            "Using blue as the background color"
        );
    }

    // Using while let
    println!();

    run(async {
        let (tx, rx) = channel();

        spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(value) = rx.recv() {
            sleep(Duration::from_millis(1000));

            println!("{} ", value);
        }

    });
}