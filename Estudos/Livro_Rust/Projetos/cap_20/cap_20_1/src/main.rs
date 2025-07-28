use std::slice;

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
}
