use std::{
    io::stdin
};

mod configuracoes_das_opcoes_da_senha;
mod limpador_do_terminal_bash;

use configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções;
use limpador_do_terminal_bash::limpar_o_terminal_bash;

fn main() {
    limpar_o_terminal_bash();

    let mut configuração_da_opção_de_senha = ConfiguraçãoDasOpções::new();

    configuração_da_opção_de_senha.get_mostrador_de_opções();

    loop {        
        trocar_a_opção_x(
            &mut configuração_da_opção_de_senha
        );
    }
}

fn trocar_a_opção_x(
    configuração_da_opção_de_senha: &mut ConfiguraçãoDasOpções
) {
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
                    let total_de_opções: u8 = 5;

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
                                    5
                                );

                                limpar_o_terminal_bash();

                                configuração_da_opção_de_senha.get_mostrador_de_opções();
                            }
                            5 => {
                                configuração_da_opção_de_senha.get_mostrador_de_opções();

                                println!(
                                    "{}\n",
                                    "não gerando ainda!"
                                )
                            }
                            _ => (),
                        }
                    } else {
                        limpar_o_terminal_bash();

                        configuração_da_opção_de_senha.get_mostrador_de_opções();

                        println!(
                            "Erro! Apenas 1 à {}!",
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