use std::{
    process::Command,
    time::{
        Duration,
        Instant
    },
    pin::{
        Pin,
        pin
    },
    thread
};

fn slow(
    name: &str,
    ms: u64
) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    // Example 1
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec! [
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, tx_fut, rx_fut];

        trpl::join_all(futures).await;
    });

    // Example 2
    println!();

    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello!" };
        let c = async { true };

        let (a_result, b_result, c_result) = trpl::join!(a, b, c);
        println!("{a_result}, {b_result}, {c_result}");
    });

    // Example 3
    println!();
    
    trpl::run(async {
        let slow = async {
            println!("'slow' started.");

            trpl::sleep(Duration::from_millis(100)).await;

            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");

            trpl::sleep(Duration::from_millis(50)).await;

            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;
    });

    // Example 4
    println!();

    trpl::run(async {
        // let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");

            slow("a", 30);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            slow("a", 10);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            slow("a", 20);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            // trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.")
        };

        let b = async {
            println!("'b' started");

            slow("b", 75);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            slow("b", 10);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            slow("b", 15);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            slow("b", 350);
            trpl::yield_now().await;
            // trpl::sleep(one_ms).await;

            // trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;
    });

    // Example 5
    println!();

    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
    
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }.await;

        let time = Instant::now() - start;

        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }.await;

        let time = Instant::now() - start;

        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    });
}
