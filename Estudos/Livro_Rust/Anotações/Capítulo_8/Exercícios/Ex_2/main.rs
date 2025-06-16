fn main() {
    let palavras = vec![
        "caqui", "arroz", "laranja", "mação", "borboleta", "pavão", "burro", "elefante"
    ];

    let vogais = vec![
        'a', 'e', 'i', 'o', 'u'
    ];

    for palavra in &palavras {
        let tamanho_da_palavra = palavra.len();
        let primeira_letra_da_palavra = &palavra[0..1];
        
        let mut primeira_letra_é_vogal = false;
        
        for vogal in &vogais {
            if primeira_letra_da_palavra == vogal.to_string() {
                primeira_letra_é_vogal = true;
            }
        }

        if primeira_letra_é_vogal {
            let palavra_mudada = format!(
                "{palavra}-hay"
            );

            println!(
                "{palavra} - {palavra_mudada}"
            );
            
        } else {
            let palavra_mudada = format!(
                "{}-{}ay", &palavra[1..tamanho_da_palavra], primeira_letra_da_palavra
            );
        
            println!(
                "{palavra} - {palavra_mudada}"
            );
        }
    }
}