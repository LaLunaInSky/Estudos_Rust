use std::{
    process::Command,
    time::Duration
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    // trpl::run(async {
    //     let fut_01 = async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     let fut_02 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     trpl::join(fut_01, fut_02).await;
    // });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];
        
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        };

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}