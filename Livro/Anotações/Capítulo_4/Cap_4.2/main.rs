fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    
    println!("The length of '{s1}' is {len}.");

    //
    let mut s2 = String::from("hello");

    println!("\nThe value of s2 is '{s2}'.");
    
    change(&mut s2);

    println!("The value of s2 is now '{s2}'.");

    //
    let mut s3 = String::from("hello");
 
    let r1 = &s3;
    let r2 = &s3;

    println!("\n{}, {}", r1, r2);

    let r3 = &mut s3;

    println!("{}", r3);

    //
    let _reference_to_nothing = dangle();

    println!("\n{}", _reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}