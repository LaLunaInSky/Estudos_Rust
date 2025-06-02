use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = &args[1].clone();
        let file_path = &args[2].clone();

        Config { query: query.to_string(), file_path: file_path.to_string() }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {0}", config.query);
    println!("In file {0}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("\nWith text:\n{contents}");
}