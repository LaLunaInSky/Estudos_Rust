mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}

    //     fn serve_order() {}

    //     fn take_payment() {}
    // }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub use crate::front_of_house::hosting;
// pub use crate::back_of_house::Appetizer;
// pub use crate::back_of_house::Breakfast;

pub use crate::back_of_house::{Appetizer, Breakfast};

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //
    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    
    println!(
        "I'd like {} toast please", meal.toast
    );

    //
    let _order_01 = Appetizer::Soup;
    let _order_02 = Appetizer::Salad;
}

fn deliver_order() {}

//
// use std::fmt;
// use std::io;

// fn function_01() -> fmt::Result {
//     // -- snip --
// }

// fn function_02() -> io::Result {
//     // -- snip --
// }

use std::fmt::Result;
use std::io::Result as IoResult;

fn function_01() -> Result {
    // -- snip --
}

fn function_02() -> IoResult<()> {
    // -- snip --
}

//

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

//
use std::collections::*;