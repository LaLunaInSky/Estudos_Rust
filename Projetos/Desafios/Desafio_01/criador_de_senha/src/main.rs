use std::{
    io::stdin,
    process::Command
};

mod configuracoes_das_opcoes_da_senha;

use configuracoes_das_opcoes_da_senha::ConfiguraçãoDasOpções;

fn main() {
    Command::new("clear").status().unwrap();

    let configuração_da_opção_de_senha = ConfiguraçãoDasOpções::new();

    loop {
        println!(
            "
 [ {:^3} ] - Números
 [ {:^3} ] - Símbolos
 [ {:^3} ] - Maiúsculas
        ",
            configuração_da_opção_de_senha.get_contém_números(),
            configuração_da_opção_de_senha.get_contém_símbolos(),
            configuração_da_opção_de_senha.get_contém_maiúsculas()
        ); 
        
        break;
    }
}