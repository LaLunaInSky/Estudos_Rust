use std::{
    io::stdin
};

use crate::recursos_terminal::{
    configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções,
    limpador_do_terminal_bash::limpar_o_terminal_bash
};

pub fn obter_um_número_inteiro(
    configuração_da_opção_de_senha: &ConfiguraçãoDasOpções
) -> u8 {
    loop {
        println!(
            "Quantas letras quer?"
        );

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(número) => {
                        if número >= 8 {
                            return número;
                        } else {
                            limpar_o_terminal_bash();

                            configuração_da_opção_de_senha.get_mostrador_de_opções();

                            println!(
                                "Erro! Apenas acima de 8!\n"
                            );
                        }
                    } 
                    Err(_) => {
                        limpar_o_terminal_bash();

                        configuração_da_opção_de_senha.get_mostrador_de_opções();

                        println!(
                            "Erro! Digite apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}