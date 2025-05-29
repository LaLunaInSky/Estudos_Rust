#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let width_1 = 30;
    // let height_1 = 50;

    //let rect_1 = (30, 50);

    let scale = 2;

    let rect_1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("rect_1 is {rect_1:#?}");

    dbg!(&rect_1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect_1.area()
    );

    if rect_1.width() {
        println!(
            "\nThe rectangle has a nonzero width; it is {}",
            rect_1.width
        );
    }

    //

    let rect_01 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_02 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect_03 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "\nCan rect_01 hold rect_02? {}", rect_01.can_hold(&rect_02)
    );

    println!(
        "Can rect_01 hold rect_03? {}", rect_01.can_hold(&rect_03)
    );

    //
    let sq = Rectangle::square(3);

    println!(
        "\nsq is {sq:?}"
    );
}