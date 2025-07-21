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

    // Using for 
    println!();

    let v_01 = vec!["a", "b", "c"];

    for (index, value) in v_01.iter().enumerate() {
        sleep(Duration::from_millis(1000));

        println!(
            "{} is at index {}",
            value, index
        );
    }

    // Using let
    println!();

    let x_01 = 5;

    let (x_02, y_01, z_01) = (1, 2, 3);

    println!(
        "x_01 = {}\nx_02 = {}\ny_01 = {}\nz_01 = {}",
        x_01, x_02, y_01, z_01
    );

    // Function Parameters
    println!();

    fn foo(_x: i32) {
        // code goes here
    }

    let point_01 = (3, 5);

    print_coordinates(&point_01);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!(
        "Current location: ({}, {})",
        x, y
    );
}