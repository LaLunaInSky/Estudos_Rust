enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    //
    let x_01: i8 = 5;
    let y_01: Option<i8> = Some(5);

    let sum = x_01 + y_01;
}