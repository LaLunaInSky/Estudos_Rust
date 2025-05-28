use std::{io, cmp::Ordering, process::Command};
use rand::Rng;

fn clean_of_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_of_terminal();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                clean_of_terminal();
                continue;
            },
        };

        if guess < 1 || guess > 100 {
            clean_of_terminal();
            println!("The secret number will be between 1 and 100.");
            continue;
        } 
        
        clean_of_terminal();
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}