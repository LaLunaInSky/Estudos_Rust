mod recursos_terminal;
mod interfaces;

use crate::recursos_terminal::{
    configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções,
    limpador_do_terminal_bash::limpar_o_terminal_bash,
    troca_das_opcoes::trocar_a_opção_x 
};

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