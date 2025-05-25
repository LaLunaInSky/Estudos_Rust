use std::{collections::HashMap, io};

struct OpçõesDoMenu {
    
}

fn limpar_o_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

fn menu_principal_de_opções_da_empresa() -> i8 {
    loop {
        limpar_o_terminal();

        println!(
            "- Bem vindo a Empresa XoXo -\n\n[ 1 ] Ver o quadro de funcinários de toda a empresa.\n[ 2 ] Ver o quadro de funcionários de um determinado setor.\n[ 3 ] Adicionar um funcionário a um setor.\n[ 4 ] Criar um novo setor na empresa.\n\nO que você gostaria?" 
        );

        let mut opção_escolhida_do_menu_principal = String::new();

        io::stdin().read_line(&mut opção_escolhida_do_menu_principal).expect("Falha ao ler o input");

        let opção_escolhida_do_menu_principal: i8 = opção_escolhida_do_menu_principal.trim().parse().expect("O input não é um número!");

        if opção_escolhida_do_menu_principal > 0 && opção_escolhida_do_menu_principal <= 4 {
            return opção_escolhida_do_menu_principal;
        }
    }
}

fn

fn main() {
    let opção_escolhida_do_menu_principal = menu_principal_de_opções_da_empresa();

    let mut mapa_dados_de_funcionários_da_empresa_por_setor = HashMap::new();

    println!("{mapa_dados_de_funcionários_da_empresa_por_setor:?}");
}