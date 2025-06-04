use std::process::Command;

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let v_01 = vec![1, 2, 3];

    let v_01_iter = v_01.iter();

    for value in v_01_iter {
        println!("Got: {value}");
    }

    //
    println!();

    let v_02: Vec<i32> = vec![1, 2, 3];

    let v_03: Vec<_> = v_02.iter().map(|x| x + 1).collect();

    println!("v_02: {:?}\nv_03: {:?}", v_02, v_03);
}