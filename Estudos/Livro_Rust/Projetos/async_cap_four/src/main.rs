use std::{
    process::Command,
    pin::pin,
    time::Duration
};
use trpl::{
    StreamExt,
    ReceiverStream,
    Stream
};

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j"
    ];

    for message in messages {
        tx.send(format!(
            "Message: '{message}'"
        )).unwrap();
    }
    
    ReceiverStream::new(rx)
}

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    // Example 1
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was:  {value}");
        }
    });

    // Example 2
    println!();

    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream
                .filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });

    // Example 3
    println!();

    trpl::run(async {
       let mut messages = get_message();

       while let Some(message) = messages.next().await {
           println!("{message}");
       }
    });
    
    // Example 4
    println!();
    
    trpl::run(async {
       let mut messages = pin!(get_message()
            .timeout(Duration::from_millis(200)));
      
       while let Some(result) = messages.next().await {
           match result {
               Ok(message) => println!("{message}"),
               Err(reason) => eprintln!("Problem: {reason:?}"),
           }
       } 
    });
}
