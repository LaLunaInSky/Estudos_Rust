use std::{
    io,
    process::Command
};

mod exercicios;

fn menu_de_opções_de_exercícios(cabeçalho_do_programa: &String) {
    loop {
        let todos_os_exercícios = vec![
            String::from("ex_001")
        ];

        println!("{}", cabeçalho_do_programa);
        
        println!("          Lista de Exercícios\n");
        
        for exercicio in todos_os_exercícios {
            print!("{exercicio} ");
        }
    
        println!("\nQual exercício você escolhe (coloque apenas o número do exercício)? ");
    
        let mut input = String::new();
    
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<u32>() {
                    Ok(number) => println!("Number is {}", number),
                    Err(_) => {
                        clean_terminal_linux();
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }

}

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    let cabeçalho_do_programa: String = String::from("- Gerenciador De Projetos De Estudo Rust -\n             Por LaLunaInSky               \n");

    menu_de_opções_de_exercícios(&cabeçalho_do_programa);

    // exercicios::ex_001::rodar_o_exercício(&cabeçalho_do_programa);
}