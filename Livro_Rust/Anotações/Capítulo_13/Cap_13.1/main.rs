use std::{
    process::Command,
    time::Duration,
    thread
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn return_mensage(user: Option<ShirtColor>, giveaway: ShirtColor) {
    println!(
        "The user with preference {:?} gets {:?}",
        user, giveaway
    );
}

fn main() {
    clean_terminal();

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_01 = Some(ShirtColor::Red);
    let giveaway_01 = store.giveaway(user_pref_01);

    return_mensage(
        user_pref_01, giveaway_01
    );

    let user_pref_02 = None;
    let giveaway_02 = store.giveaway(user_pref_02);

    return_mensage(
        user_pref_02, giveaway_02
    );

    //
    println!();

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");

        thread::sleep(Duration::from_secs(2));

        println!("{num}");
        
        num
    };

    expensive_closure(5);

    //
    println!();

    let list_01 = vec![1, 2, 3];
    println!("Before defining closure: {list_01:?}");

    let only_borrows_01 = || println!("From closure: {list_01:?}");

    println!("Before calling closure: {list_01:?}");
    only_borrows_01();
    println!("After calling closure: {list_01:?}");

    println!();

    let mut list_02 = vec![1, 2, 3];
    println!("Before defining closure: {list_02:?}");

    let mut borrows_mutably = || list_02.push(7);

    borrows_mutably();
    println!("After calling closure: {list_02:?}");

    //
    println!();

    let list_03 = vec![1, 2, 3];
    println!("Before defining closure: {list_03:?}");

    thread::spawn(move || println!("From thread: {list_03:?}"))
        .join()
        .unwrap();
}