use std::slice;

// 20-8
unsafe extern "C" {
    // 20-9
    safe fn abs(input: i32) -> i32;
}

// call function rust in other language
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!(
        "Just called a Rust function from C!"
    );
}

// 20-10 - Security
static HELLO_WORLD: &str = "Hello, World!";

// 20-11 - Unsecurity
static mut COUNTER: u32 = 0;

// SAFETY: Calling this from more than a single thread at a time is undefined behavior, so you *must* guarantee you only call it from a single thread at a time
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

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

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // 20-4
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!(
        "\n{:?}\n{:?}",
        a, b
    );

    // 20-5
    fn split_at_mut(
        values: &mut [i32], mid: usize
    ) -> (
        &mut [i32],
        &mut [i32]
    ) {
        let len = values.len();
        
        assert!(mid <= len);

        // 20-6
        let ptr = values.as_mut_ptr();

        unsafe {
            (
                slice::from_raw_parts_mut(
                    ptr, mid
                ),
                slice::from_raw_parts_mut(
                    ptr.add(mid), len - mid
                ),
            )
        }
    }

    // 20-8
    unsafe {
        println!(
            "\nAbsolute value of -3 according to C: {}",
            abs(-3)
        );
    }

    // 20-9
    println!(
        "\nAbsolute value of -3 according to C: {}",
        abs(-3)
    );

    // 20-10
    println!(
        "\nName is: {}",
        HELLO_WORLD
    );

    // 20-11
    unsafe {
        // SAFETY: This is only called from a single thread in `main`
        add_to_count(3);

        println!(
            "\nCOUNTER: {}",
            *(&raw const COUNTER)
        );
    }
}