fn main() {
    // 20-1
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    // 20-2
    let address = 0x012345usize;
    let r = address as *const i32;

    //20-3
    unsafe {
        println!(
            "r1 is: {}\nr2 is: {}",
            *r1, *r2
        );
    }
}
