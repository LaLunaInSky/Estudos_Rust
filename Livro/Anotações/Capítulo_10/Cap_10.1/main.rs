use std::{process::Command, cmp::PartialOrd};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[derive(Debug)]
struct PointGeneric<T, U> {
    x: T,
    y: U,
}

fn clean_of_terminal() {
    Command::new("clear").status().unwrap();
}

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }


// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    clean_of_terminal();

    let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest_i32(&number_list);

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);

    let result = largest(&char_list);
    println!("The largest char is {result}");

    //
    println!();

    let point_one_type = Point { x: 5, y: 10 };
    let point_two_type = PointGeneric { x: 1, y: 4.2 };

    println!("x = {} \ny = {}\n", point_one_type.x(), point_one_type.y);
    println!("x = {} \ny = {}", point_two_type.x, point_two_type.y);
}