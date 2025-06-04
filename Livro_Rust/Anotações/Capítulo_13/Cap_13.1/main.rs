use std::process::Command;

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
}