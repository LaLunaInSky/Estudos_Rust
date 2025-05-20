#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!(
    //         "The maximum is configured to be {max}"
    //     ),
    //     _ => (),
    // }

    if let Some(max) = config_max {
        println!(
            "The maximum is configured to be {max}"
        )
    };

    //
    println!();

    let mut count = 0;
    
    let mut coin = Coin::Penny;
    
    value_in_cents(
        coin, count
    );

    coin = Coin::Nickel;

    value_in_cents(
        coin, count
    );

    coin = Coin::Dime;

    value_in_cents(
        coin, count
    );

    coin = Coin::Quarter(UsState::Alabama);

    value_in_cents(
        coin, count
    );

    coin = Coin::Quarter(UsState::Alaska);

    value_in_cents(
        coin, count
    );

    println!(
        "The count is {}", count
    );
}

fn value_in_cents(coin: Coin, count: &mut u32) {
    match coin {
        Coin::Quarter(state) => println!(
            "State quarter from {state:?}!"
        ),
        _ => *count += 1,
    }
}