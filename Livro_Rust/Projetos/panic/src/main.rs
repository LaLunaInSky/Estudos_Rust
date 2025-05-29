use std::process::Command;

fn main() {
    limpar_o_terminal();

    //panic!("crash and burn");

    let v_01 = vec![1, 2, 3];

    v_01[99];
}

fn limpar_o_terminal() {
    Command::new("clear").status().unwrap();
}