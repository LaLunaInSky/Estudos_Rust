use std::{
    process::Command,
    mem::drop,
};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping CustomSmartPointer with data `{}`!", self.data
        );
    }
}

fn main() {
    clean_terminal();

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    //
    println!();

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created.");

    drop(e);

    println!("CustomSmartPointer dropped before the end of main.");
}