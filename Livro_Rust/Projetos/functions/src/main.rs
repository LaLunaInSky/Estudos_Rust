fn return_of_value_five() -> i32 {
    5
}


fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = return_of_value_five();

    println!("The value of x is: {x}");

    let z = plus_one(5);

    println!("The value of z is: {z}");
}


fn plus_one(z: i32) -> i32 {
    z + 1
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}