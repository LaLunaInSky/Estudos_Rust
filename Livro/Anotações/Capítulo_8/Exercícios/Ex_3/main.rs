use std::{collections::HashMap, io};

fn main() {
    let nomes_dos_funcionários: [&str; 5] = [
        "julia", "ana", "luiz", "joão", "lucas"
    ];

    let mut mapa_de_funcionários_da_empresa = HashMap::new();

    let setores_da_empresa = vec![
        "administração", "financeiro", "suporte técnico", "rh", "atendimento ao público", "venda"
    ];

    for setor in &setores_da_empresa {
        mapa_de_funcionários_da_empresa.insert(
            setor, Vec::<String>::new()
        );
    };

    for value in mapa_de_funcionários_da_empresa.values_mut() {
        for nome in &nomes_dos_funcionários {
            value.push(nome.to_string());
        }
    }

    // println!(
    //     "{mapa_de_funcionários_da_empresa:?}"
    // );

    let opção_escolhida = escolher_setor_da_empresa(&setores_da_empresa);

    println!();

    for (key, values) in &mapa_de_funcionários_da_empresa {
        if **key == setores_da_empresa[opção_escolhida] {
            mostrar_funcionários_do_setor_n(setores_da_empresa[opção_escolhida], values);
        } 
    }
}

fn mostrar_todos_os_setores_da_empresa() {
    
}

fn escolher_setor_da_empresa(vec_nome_dos_setores: &Vec<&str>) -> usize {
    loop {
        let mut quantidade_de_setores = 1;

        for setor in vec_nome_dos_setores {
            println!(
                "[ {quantidade_de_setores} ] {setor}"
            );

            quantidade_de_setores += 1;
        }

        println!(
            "Qual setor você quer ver? "
        );

        let mut opção_escolhida_do_menu = String::new();

        io::stdin().read_line(&mut opção_escolhida_do_menu).expect("Falha ao ler o input");

        let opção_escolhida_do_menu: usize = opção_escolhida_do_menu.trim().parse().expect("Falha ao transferir string em número.");

        if opção_escolhida_do_menu > 0 && opção_escolhida_do_menu <= 6 {
            return opção_escolhida_do_menu - 1;
        }
    }
}

fn mostrar_funcionários_do_setor_n(nome_do_setor: &str, vector_de_nome: &Vec<String>) {
    println!{
        "-- Setor {} --", nome_do_setor
    };

    for value in vector_de_nome {
        println!(
            "° {value}"
        );
    }
}