use std::{process::{self, Command}, env};

use minigrep::{Config, run};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);

    if let Err(e) = run(config) {
        println!("\nApplication error: {e}");
        process::exit(1);
    }
}