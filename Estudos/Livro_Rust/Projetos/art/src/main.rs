use art::{
    PrimaryColor,
    mix,
};

fn main() {
    let color_red = PrimaryColor::Red;
    let color_yellow = PrimaryColor::Yellow;

    mix(color_red, color_yellow);
}