use std::{
    process::Command,
};

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();
    
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);
        
        while let Some(value) = stream.next().await {
            pritnln!("The value was:  {value}");
        }
    });
}