use std::{
    fmt,
    ops::Add
};

// 20-15
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 20-16
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 20-17
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!(
            "This is your captain speaking."
        );
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!(
            "Up!"
        );
    }
}

impl Human {
    fn fly(&self) {
        println!(
            "*waving arms furiously*"
        );
    }
}

// 20-20
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 20-23
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();

        let len = output.len();

        println!(
            "{}\n*{}*\n* {} *\n*{}*\n{}",
            "*".repeat(len + 4),
            " ".repeat(len + 2),
            output,
            " ".repeat(len + 2),
            "*".repeat(len + 4)
        )
    }
}

// 20-24
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(
        &self, f: &mut fmt::Formatter
    ) -> fmt::Result {
        write!(
            f, "[{}]",
            self.0.join(", ")
        )
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // 20-18
    let person = Human;
    person.fly();

    // 20-19
    Pilot::fly(&person);
    Wizard::fly(&person);

    // 20-20
    println!(
        "\nA baby dog is called a {}",
        Dog::baby_name()
    );

    // 20-22
    println!(
        "\nA baby dog is called a {}",
        <Dog as Animal>::baby_name()
    );

    // 20-24
    let w = Wrapper(
        vec![
            String::from("hello"),
            String::from("world!")    
        ]
    );

    println!(
        "\nw = {w}"
    );
}