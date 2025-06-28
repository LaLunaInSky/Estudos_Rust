use std::io::{self, Read};

fn descrição_do_exercícios() {
    println!("Descrição do exercício 001:\n");
    println!(
        "Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos."
    );
}

pub fn rodar_o_exercício() {
    descrição_do_exercícios();

    println!();

    for indice in 1..3 {
        obter_a_entrada_de_um_número_inteiro(indice);
    }

    println!();
}

fn soma_de_dois_números_inteiros(primeiro_número: i32, segundo_número: i32) -> i32 {
    primeiro_número + segundo_número
}

fn obter_a_entrada_de_um_número_inteiro(indice_da_chamada_do_input: i32) {
    println!(
        "Digite o {indice_da_chamada_do_input}º número inteiro: "
    );

    let mut número_digitado = String::new();

    io::stdin().read_line(&mut número_digitado).unwrap();

    match número_digitado.parse::<i32>() {
        Ok(n) => println!("Parsed value: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    println!("{número_digitado}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soma_de_dois_números() {
        let primeiro_número = 5;
        let segundo_número = 5;

        let resultado_da_soma = soma_de_dois_números_inteiros(primeiro_número, segundo_número);

        assert_eq!(resultado_da_soma, 10);
    }
}