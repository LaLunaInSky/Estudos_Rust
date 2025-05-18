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
            "The rectangle has a nonzero width; it is {}",
            rect_1.width
        );
    }
}