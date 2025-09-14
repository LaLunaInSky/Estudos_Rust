// use std::{
//     io::stdin
// };

mod configuracoes_das_opcoes_da_senha;
mod limpador_do_terminal_bash;
mod troca_das_opcoes;
mod obtencao_de_numero_inteiro;

use configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções;
use limpador_do_terminal_bash::limpar_o_terminal_bash;
use troca_das_opcoes::trocar_a_opção_x;

fn main() {
    limpar_o_terminal_bash();

    let mut configuração_da_opção_de_senha = ConfiguraçãoDasOpções::new();

    configuração_da_opção_de_senha.get_mostrador_de_opções();

    loop {        
        let resposta_opções = trocar_a_opção_x(
            &mut configuração_da_opção_de_senha
        );

        if !resposta_opções {
            break;
        }
    }
}