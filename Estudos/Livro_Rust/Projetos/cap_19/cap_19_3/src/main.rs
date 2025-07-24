fn main() {
    let x_01 = 1;
    
    match x_01 {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    //
    println!();

    let x_02 = Some(5);
    let y_01 = 10;

    match x_02 {
        Some(50) => println!("Got 50"),
        Some(y_01) => println!("Matched,  y = {}", y_01),
        _ => println!("Default case, x = {:?}", x_02),
    }

    println!("at the end: x = {:?}, y = {}", x_02, y_01);

    //
    println!();

    let x_03 = 1;

    match x_03 {
        1 | 2 => println!("One or Two!"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    //
    println!();

    let x_04 = 4;

    match x_04 {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    //
    println!();

    let x_05 = 'c';

    match x_05 {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring Structs
    println!();

    let p_01 = Point{
        x: 0, y: 7
    };

    let Point { x: a, y: b } = p_01;

    let Point { x, y } = p_01;
    
    println!(
        "a == 0 ? {}\nb == 7 ? {}\n",
        a == 0,
        b == 7
    );

    println!(
        "x == 0 ? {}\ny == 7 ? {}\n",
        x == 0,
        y == 7
    );

    match p_01 {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums
    println!();

    let msg_01 = Message_01::ChangeColor(
        0, 160, 255
    );

    match msg_01 {
        Message_01::Quit => {
            println!(
                "The quit variant has no data to desctructure."
            );
        }
        Message_01::Move { x, y } => {
            println!(
                "Move in the x direction {x} and in the y direction {y}"
            );
        }
        Message_01::Write(text) => {
            println!(
                "The message: {text}"
            );
        }
        Message_01::ChangeColor(r, g, b) => {
            println!(
                "Change color to red {r}, green {g}, and blue {b}"
            );
        }
    }

    // Destructuring Nested Structs and Enums
    println!();

    let msg_02 = Message_02::ChangeColor(Color::Hsv(0, 160, 255));

    match msg_02 {
        Message_02::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color to red {r}, green {g}, and blue {b}"
            );
        }
        Message_02::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color to hue {h}, saturation {s}, value {v}"
            );
        }
        _ => (),
    }

    // Destructuring Structs and Tuples
    println!();

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Entire value with _
    foo(3, 4);

    // Parts of a value with a Nested _
    // Example 1
    println!();

    let mut setting_value_01 = Some(5);
    let new_setting_value = Some(10);

    match (setting_value_01, new_setting_value) {
        (Some(_), Some(_)) => {
            println!(
                "Can't overwrite an existing customized value"
            );
        }
        _ => {
            setting_value_01 = new_setting_value;
        }
    }

    println!("setting is {setting_value_01:?}");

    // Example 2
    println!();

    let numbers_01 = (2, 4, 8, 16, 32);

    match numbers_01 {
        (first, _, third, _, fifth) => {
            println!(
                "Some numbers: {first}, {third}, {fifth}"
            );
        }
    }

    // Unused variable by starting its name with _
    println!();

    let _x_06 = 5;
    let y_02 = 10;

    let s = Some(
        String::from("Hello!")
    );

    if let Some(_) = s {
        println!("found a string")
    };

    println!("{s:?}");

    // Remaining parts of a value with ..
    println!();

    struct Point_02 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point_02 { x: 0, y: 0, z: 0 };

    match origin  {
        Point_02 { x, .. } => println!("x is {x}"),
    }

    match numbers_01 {
        (first, .., last ) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Extra conditionals with match guards
}

// Destructuring Structs
struct Point {
    x: i32,
    y: i32,
}

// Destructuring Enums
enum Message_01 {
    Quit,
    Move {
        x: i32, 
        y:i32 
    },
    Write(String),
    ChangeColor(
        i32, 
        i32,
        i32
    ),
}

// Destructuring Nested Structs and Enums
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message_02 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// Entire value with _
fn foo(_: i32, y: i32) {
    println!(
        "This code only uses the y parameter: {y}"
    );
}