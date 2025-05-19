#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!(
                "State quarter from {state:?}!"
            );
            25
        }
    }
}

fn main() {
    let my_coin_01 = value_in_cents(Coin::Penny);
    println!("My coin value is {}.", my_coin_01);

    let my_coin_02 = value_in_cents(Coin::Nickel);
    println!("My coin value is {}.", my_coin_02);

    let my_coin_03 = value_in_cents(Coin::Dime);
    println!("My coin value is {}.", my_coin_03);

    let my_coin_04 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("My coin value is {}.", my_coin_04);

    let my_coin_05 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("My coin value is {}.", my_coin_05);

    //
    println!();
    let five = Some(5);
    let six  = plus_one(five);
    let none = plus_one(None);

    println!(
        "Five variable is: {five:?}\nSix variable is: {six:?}\nNone variable is: {none:?}"
    );
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}