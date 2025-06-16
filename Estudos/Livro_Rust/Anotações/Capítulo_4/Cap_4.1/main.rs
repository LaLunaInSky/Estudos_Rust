fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{s}");

    //
    let x = 5;
    let y = x;

    println!{"\nx = {x}\ny = {y}"};

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("\ns1 = {s1}\ns2 = {s2}");

    //
    let mut s3 = String::from("hello");
    println!("\n{s3}, world!");

    s3 = String::from("ahoy");
    println!("{s3}, world!");

    //
    let s4 = String::from("hello");

    takes_ownership(s4);

    let z = 5;

    makes_copy(z);

    println!("{}", z);

    //
    let _s5 = gives_ownership();

    let s6 = String::from("hello");

    let _s7 = takes_and_gives_back(s6.clone());

    println!("\ns5 = {_s5}\ns6 = {s6}\ns7 = {_s7}");

    //
    let s8 = s6.clone();
    
    let (s9, len) = calculate_length(s8);
    
    println!("\nThe length of '{s9}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("\n{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}