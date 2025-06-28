fn descrição_do_exercícios() {
    println!("Descrição do exercício 001:\n");
    println!(
        "Um programa que lê dois números inteiro e\nmostra a soma entre os mesmos."
    );
}

pub fn rodar_o_exercício() {
    descrição_do_exercícios();

    println!();

    obter_a_entrada_de_um_número_inteiro(1);
}

fn soma_de_dois_números_inteiros(primeiro_número: i32, segundo_número: i32) -> i32 {
    primeiro_número + segundo_número
}

fn obter_a_entrada_de_um_número_inteiro(indice_da_chamada_do_input: i32) {
    println!(
        "Digite o {indice_da_chamada_do_input}º número inteiro: "
    );
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