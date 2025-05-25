use std::collections::HashMap;

fn main() {
    let mut números_inteiros_vec = vec![5,11,36,74,8,4,2,68,45,12,44,56,99,41,25,36,52,55,33,41,2,36,44,11,26,59,54,64,31,70];

    números_inteiros_vec.sort();

    println!(
        "{números_inteiros_vec:?}"
    );

    let metade_da_quantidade_total_obtida = (números_inteiros_vec.len()) / 2;

    println!(
        "A número inteiro na posição médiana da lista é o {}.", números_inteiros_vec[metade_da_quantidade_total_obtida - 1]
    );

    //
    println!();

    let mut mapa_da_quantidade_de_cada_númeor_na_lista = HashMap::new();
    
    for número in &números_inteiros_vec {
        let quantidade = mapa_da_quantidade_de_cada_númeor_na_lista.entry(número.to_string()).or_insert(0);
        
        *quantidade += 1;
    }

    let mut maior_quantidade_de_vezes = &{0};
    let mut número_que_mais_aparece = "";

    for (key, value) in &mapa_da_quantidade_de_cada_númeor_na_lista {
        if value > maior_quantidade_de_vezes {
            maior_quantidade_de_vezes = value;
            número_que_mais_aparece = key;
        }
    }

    println!(
        "O número que mais apareceu na lista foi o número {número_que_mais_aparece}, que apareceu {maior_quantidade_de_vezes} vezes."
    );
}