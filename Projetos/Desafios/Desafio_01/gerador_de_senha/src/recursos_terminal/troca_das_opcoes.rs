use std::{
    io::stdin
};

use crate::recursos_terminal::{
    configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções,
    obtencao_de_numero_inteiro::obter_um_número_inteiro,
    limpador_do_terminal_bash::limpar_o_terminal_bash
};

use crate::gerador_de_senha::gerador_de_senha;

pub fn trocar_a_opção_x(
    configuração_da_opção_de_senha: &mut ConfiguraçãoDasOpções
) -> bool {
    loop{
        println!{
            "Qual a opção?"
        };

        let mut input = String::new();

        match stdin().read_line(
            &mut input
        ) {
            Ok(_) => {
                match input.trim().parse::<u8>() {
                    Ok(opção) => {
                        let total_de_opções: u8 = 6;

                        if opção > 0 && opção < total_de_opções + 1 {
                            limpar_o_terminal_bash();

                            match opção {
                                1 => {
                                    configuração_da_opção_de_senha.set_contém_números();
                                    
                                    configuração_da_opção_de_senha.get_mostrador_de_opções();
                                }
                                2 => {
                                    configuração_da_opção_de_senha.set_contém_símbolos();
                                    
                                    configuração_da_opção_de_senha.get_mostrador_de_opções();
                                }
                                3 => {
                                    configuração_da_opção_de_senha.set_contém_maiúsculas();
                                    
                                    configuração_da_opção_de_senha.get_mostrador_de_opções();
                                }
                                4 => {
                                    configuração_da_opção_de_senha.get_mostrador_de_opções();

                                    configuração_da_opção_de_senha.set_total_de_letras(
                                        obter_um_número_inteiro(
                                            &configuração_da_opção_de_senha
                                        )
                                    );

                                    limpar_o_terminal_bash();

                                    configuração_da_opção_de_senha.get_mostrador_de_opções();
                                }
                                5 => {
                                    configuração_da_opção_de_senha.get_mostrador_de_opções();

                                    let senha_gerada = gerador_de_senha(
                                        configuração_da_opção_de_senha.get_bool_contém_números(),
                                        configuração_da_opção_de_senha.get_bool_contém_símbolos(),
                                        configuração_da_opção_de_senha.get_bool_contém_maiúsculas(),
                                        configuração_da_opção_de_senha.get_total_de_letras()
                                    );

                                    println!(
                                        "{}\n",
                                        senha_gerada
                                    )
                                }
                                6 => return false,
                                _ => return false,
                            }
                        } else {
                            limpar_o_terminal_bash();

                            configuração_da_opção_de_senha.get_mostrador_de_opções();

                            println!(
                                "Erro! Apenas 1 à {}!\n",
                                total_de_opções
                            );
                        }
                    }
                    Err(_) => {
                        limpar_o_terminal_bash();

                        configuração_da_opção_de_senha.get_mostrador_de_opções();

                        println!(
                            "Erro! Apenas números!\n"
                        );
                    }
                }
            }
            Err(_) => ()
        }
    }
}