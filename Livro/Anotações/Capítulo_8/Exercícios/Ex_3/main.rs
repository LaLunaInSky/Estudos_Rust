use std::{collections::HashMap, process::Command, io};

fn main() {
    // limpar_o_terminal();

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

    // mostrar_menu_principal_de_opções_da_empresa(&setores_da_empresa, &mut mapa_de_funcionários_da_empresa);

}

fn limpar_o_terminal() {
    Command::new("clear").status().unwrap();
}

fn mostrar_opções_dos_setores_da_empresa(vec_nome_dos_setores: &Vec<&str>) -> usize {
    loop {
        let mut quantidade_de_setores = 1;
    
        for setor in vec_nome_dos_setores {
            println!(
                "[ {quantidade_de_setores} ] {setor}"
            );
    
            quantidade_de_setores += 1;
        }
        
        println!(
            "Qual setor você quer escolhe? "
        );

        let mut opção_escolhida_do_menu = String::new();

        io::stdin().read_line(&mut opção_escolhida_do_menu).expect("Falha ao ler o input");

        let mut opção_escolhida_do_menu: usize = opção_escolhida_do_menu.trim().parse().expect("Falha ao transferir string em número.");

        if opção_escolhida_do_menu > 0 && opção_escolhida_do_menu <= 6 {
            opção_escolhida_do_menu -= 1;
            return opção_escolhida_do_menu;
        } else {
            limpar_o_terminal();
        }
    }
}

fn mostrar_menu_principal_de_opções_da_empresa(vec_nome_dos_setores: &Vec<&str>, mapa_da_empresa: &mut HashMap<&&str, Vec<String>>) {
    loop {
        println!("[ 1 ] Ver Todos Os Funcionários da Empresa, separados por setor");
        println!("[ 2 ] Ver Todos os Funcionários de um setor da Empresa");
        println!("[ 3 ] Adicionar um Novo Funcionário em um Setor da Empresa");
        println!("[ 4 ] Encerrar O Programa");

        let mut opção_escolhida_do_menu_principal = String::new();

        io::stdin().read_line(&mut opção_escolhida_do_menu_principal).expect("Falha ao ler o input");

        let opção_escolhida_do_menu_principal: usize = opção_escolhida_do_menu_principal.trim().parse().expect("Falha ao converter o input lido");

        if opção_escolhida_do_menu_principal > 0 && opção_escolhida_do_menu_principal < 5 {
            limpar_o_terminal();

            if opção_escolhida_do_menu_principal == 1 {
                mostrar_todos_os_funcionarios_da_empresa_separados_por_setor(mapa_da_empresa);
                
            } else if opção_escolhida_do_menu_principal == 2 {
                escolher_setor_da_empresa_para_ver(vec_nome_dos_setores, mapa_da_empresa);
            
            } else if opção_escolhida_do_menu_principal == 3 {
                adicionar_novo_funcionário_em_um_setor_n(vec_nome_dos_setores, mapa_da_empresa);
            
            } else if opção_escolhida_do_menu_principal == 4 {
                break;
            }
        }    
    }
}

fn adicionar_novo_funcionário_em_um_setor_n(vec_nome_dos_setores: &Vec<&str>, mapa_da_empresa: &mut HashMap<&&str, Vec<String>>) {
    let opção_escolhida_do_menu_de_setores: usize = mostrar_opções_dos_setores_da_empresa(vec_nome_dos_setores);

    limpar_o_terminal();

    println!("Qual o nome do funcionário que vai ser adicionado no setor {}?", vec_nome_dos_setores[opção_escolhida_do_menu_de_setores]);

    let mut nome_do_novo_funcionário = String::new();

    io::stdin().lock().read_line(&mut nome_do_novo_funcionário).unwrap();

    limpar_o_terminal();

    println!("O funcionário {nome_do_novo_funcionário} foi adicionado com sucesso no setor {}!", vec_nome_dos_setores[opção_escolhida_do_menu_de_setores]);

    println!();

    for (key, values) in mapa_da_empresa {
        if ***key == *vec_nome_dos_setores[opção_escolhida_do_menu_de_setores] {
            mostrar_funcionários_do_setor_n(vec_nome_dos_setores[opção_escolhida_do_menu_de_setores], values);
        } 
    };
}

fn mostrar_todos_os_funcionarios_da_empresa_separados_por_setor(mapa_da_empresa: &mut HashMap<&&str, Vec<String>>) {    
    for (key, values) in mapa_da_empresa {
        mostrar_funcionários_do_setor_n(key, values);
        
        println!();
    }
}

fn escolher_setor_da_empresa_para_ver(vec_nome_dos_setores: &Vec<&str>, mapa_da_empresa: &mut HashMap<&&str, Vec<String>>) {    
    let opção_escolhida_do_menu_de_setores: usize = mostrar_opções_dos_setores_da_empresa(vec_nome_dos_setores);

    limpar_o_terminal();
    
    for (key, values) in mapa_da_empresa {
        if ***key == *vec_nome_dos_setores[opção_escolhida_do_menu_de_setores] {
            mostrar_funcionários_do_setor_n(vec_nome_dos_setores[opção_escolhida_do_menu_de_setores], values);
        } 
    }
    
    println!();
}

fn mostrar_funcionários_do_setor_n(nome_do_setor: &str, vector_de_nome: &mut Vec<String>) {
    println!{
        "-- Setor {} --", nome_do_setor
    };

    for value in vector_de_nome {
        println!(
            "° {value}"
        );
    }
}