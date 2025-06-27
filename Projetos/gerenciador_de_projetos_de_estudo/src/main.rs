use std::process::Command;

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    let frase_01: String = String::from("- Gerenciador De Projetos De Estudo Rust -");
    let limite_de_caracter: usize = frase_01.len();

    let frase_02: String = String::from("Por LaLunaInSky");
    let tamanho_frase_02: usize = frase_02.len();
    let quanto_falta_para_o_limite_de_caracter: usize = limite_de_caracter - tamanho_frase_02;

    println!("{}\n{}", frase_01, frase_02);
    println!();
    println!("{} - {} - {}", limite_de_caracter, tamanho_frase_02, quanto_falta_para_o_limite_de_caracter);
}