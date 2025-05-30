use std::process::Command;

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    let x = 5;

    let r = &x;

    println!("r: {r}");

    //
    println!();

    let string_01 = String::from("abcd");
    let string_02 = "xyz".to_string();

    let result = longest(&string_01, &string_02);
    println!("The longest string is {result}");
}

fn longest<'a>(text_01: &'a String, text_02: &'a String) -> &'a String {
    if text_01.len() > text_02.len() {
        text_01
    } else {
        text_02
    }
}