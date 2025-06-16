#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        UsState::Alabama => year >= 1819,
        UsState::Alaska => year >= 1959,
        // -- snip --
    }
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
    let mut count = 0;
    
    let mut coin = Coin::Penny;

    // match coin {
    //     Coin::Quarter(state) => println!(
    //         "State quarter from {state:?}!"
    //     ),
    //     _ => count + 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!(
            "State quarter from {state:?}!"
        );
    } else {
        count += 1;
    }

    coin = Coin::Nickel;

    // match coin {
    //     Coin::Quarter(state) => println!(
    //         "State quarter from {state:?}!"
    //     ),
    //     _ => count + 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!(
            "State quarter from {state:?}!"
        );
    } else {
        count += 1;
    }

    coin = Coin::Dime;

    // match coin {
    //     Coin::Quarter(state) => println!(
    //         "State quarter from {state:?}!"
    //     ),
    //     _ => count + 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!(
            "State quarter from {state:?}!"
        );
    } else {
        count += 1;
    }

    coin = Coin::Quarter(UsState::Alabama);

    // match coin {
    //     Coin::Quarter(state) => println!(
    //         "State quarter from {state:?}!"
    //     ),
    //     _ => count + 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!(
            "State quarter from {state:?}!"
        );
    } else {
        count += 1;
    }
    
    coin = Coin::Quarter(UsState::Alaska);

    // match coin {
    //     Coin::Quarter(state) => println!(
    //         "State quarter from {state:?}!"
    //     ),
    //     _ => count + 1,
    // }

    if let Coin::Quarter(state) = coin {
        println!(
            "State quarter from {state:?}!"
        );
    } else {
        count += 1;
    }

    println!("Count final is {count}");

    //
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!(
    //             "{state:?} is pretty old, for America!"
    //         ))
    //     } else {
    //         Some(format!(
    //             "{state:?} is relatively new."
    //         ))
    //     }
    // } else {
    //     None
    // }

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!(
            "{state:?} is pretty old, for America!"
        ))
    } else {
        Some(format!(
            "{state:?} is relatively new."
        ))
    }
}