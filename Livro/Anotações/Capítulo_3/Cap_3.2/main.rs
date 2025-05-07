use std::io;

fn main() {
    let a: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Por favor entre com a posição do Array.");

    let mut posição: String = String::new();

    io::stdin()
        .read_line(&mut posição)
        .expect("Falha ao ler a linha");

    let posição: usize = posição
        .trim()
        .parse()
        .expect("Posição informada não é um número");

    let elemento = a[posição];

    println!("O valor do elemento na posição {posição} é: {elemento}")
}