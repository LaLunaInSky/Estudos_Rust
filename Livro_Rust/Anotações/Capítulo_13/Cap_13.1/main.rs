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
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
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

fn return_mensage(user: Option<ShirtColor>, give_away: ShirtColor) {
    println!(
        "The user with preference {:?} gets {:?}",
        user, give_away
    );
}

fn main() {
    clean_terminal();

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_01 = Some(ShirtColor::Red);
    let give_away_01 = store.give_away(user_pref_01);

    return_mensage(
        user_pref_01, give_away_01
    );

    let user_pref_02 = None;
    let give_away_02 = store.give_away(user_pref_02);

    return_mensage(
        user_pref_02, give_away_02
    );
}