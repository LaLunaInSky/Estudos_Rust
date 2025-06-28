use std::process::Command;

mod exercicios;

// mod exercícios {
//     pub mod ex_001 {
//         fn descrição_do_exercícios() {
//             println!("Descrição do exercício 001");
//         }

//         pub fn rodar_o_exercício() {
//             descrição_do_exercícios();
//         }
//     }
// }

fn clean_terminal_linux() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal_linux();

    let cabeçalho_do_programa: String = String::from("- Gerenciador De Projetos De Estudo Rust -\n             Por LaLunaInSky               \n");

    println!("{}", cabeçalho_do_programa);

    exercicios::ex_001::rodar_o_exercício(&cabeçalho_do_programa);
}