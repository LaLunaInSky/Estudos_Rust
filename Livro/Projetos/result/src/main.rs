use std::{process::Command, fs::File, io::{self, ErrorKind, Read}};

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    limpar_o_terminal();

    // let greeting_file_result = File::open("hello_01.txt");

    // let greeting_file_01 = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello_01.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         _ => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };

    // or
    let greeting_file_01 = File::open("hello_01.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_01.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // unwarp e expect
    //let greeting_file_02 = File::open("hello_02.txt").unwrap();
    let greeting_file_02 = File::open("hello_01.txt").expect("hello_01.txt should be included in this project");

    //
    let username = read_username_from_file();

    println!("{username:?}")
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello_03.txt");
    
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // macth username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // or 
    let mut username_file = File::open("hello_01.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    
    Ok(username)
}

fn limpar_o_terminal() {
    Command::new("clear").status().unwrap();
}