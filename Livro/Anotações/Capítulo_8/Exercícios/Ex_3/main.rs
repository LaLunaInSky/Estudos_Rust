use std::collections::HashMap;

fn main() {
    let mut mapa_de_funcionários_da_empresa = HashMap::new();

    let setores_da_empresa = vec![
        "administração", "financeiro", "suporte técnico", "rh", "atendimento ao público", "venda"
    ];

    for setor in &setores_da_empresa {
        mapa_de_funcionários_da_empresa.entry(setor).or_insert(0);
    };

    println!(
        "{mapa_de_funcionários_da_empresa:?}"
    );

    println!(
        "{setores_da_empresa:?}"
    );
}