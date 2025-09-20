mod numeros;
mod alfabeto;
mod simbolos;
mod alfabeto_maiusculo;

use numeros::Números;
use alfabeto::Alfabeto;
use simbolos::Símbolos;
use alfabeto_maiusculo::AlfabetoMaiúsculo;

use rand::random_range;

pub fn gerador_de_senha(

    contém_números: bool,
    contém_símbolos: bool,
    contém_maiúsculas: bool,
    total_de_letras: u8

) -> String {

    let mut senha_gerada = String::new();
    let mut quantidade_de_opções: usize = 1;
    
    let alfabeto = Alfabeto::new();
    let alfabeto_maiúsculo = AlfabetoMaiúsculo::new();
    let números = Números::new();
    let símbolos = Símbolos::new();

    if contém_números {
        quantidade_de_opções += 1;
    }

    if contém_símbolos {
        quantidade_de_opções += 1;
    }

    for letra in 0..total_de_letras {
        let mut sorteio_de_tamanho_letra = 1;

        if contém_maiúsculas {
            sorteio_de_tamanho_letra = random_range(0..2);
        }

        let letra_sorteada: char = if sorteio_de_tamanho_letra == 0 {
            alfabeto_maiúsculo.get_alfabeto_maiúsculo(
                random_range(
                    0..alfabeto_maiúsculo.get_quantidade_de_letras()
                )
            )
        } else {
            alfabeto.get_alfabeto(
                random_range(
                    0..alfabeto.get_quantidade_de_letras()
                )
            )
        };
        

        let mut sorteados: Vec<char> = vec![
           letra_sorteada
        ];

        if contém_números {
            sorteados.push(
                números.get_número(
                    random_range(
                        0..números.get_total_de_números()
                    )
                )
            );
        }

        if contém_símbolos {
            sorteados.push(
                símbolos.get_símbolo(
                    random_range(
                        0..símbolos.get_quantidade_de_símbolos()
                    )
                )
            );
        }
        
        let opção_sorteada = random_range(
            0..quantidade_de_opções
        );

        senha_gerada.push(
            sorteados[
                opção_sorteada
            ]
        );
    }

    return senha_gerada;
}