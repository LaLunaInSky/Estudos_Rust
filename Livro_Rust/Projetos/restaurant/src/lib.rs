mod front_of_house;
mod back_of_house;

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