

// Comece pela raiz do crate: ao compilar um crate, o compilador primeiro procura no arquivo raiz do crate (geralmente src/lib.rs para um crate de biblioteca ou src/main.rs para um crate binário) o código a ser compilado.

// Declarando módulos: No arquivo raiz do crate, você pode declarar novos módulos; digamos que você declare um módulo "jardim" com mod garden;. O compilador procurará o código do módulo nestes lugares:
// ° Em linha, dentro de chaves que substituem o ponto e vírgula a seguirmod garden
// ° No arquivo src/garden.rs
// ° No arquivo src/garden/mod.rs

// Declarando submódulos: Em qualquer arquivo que não seja o raiz do crate, você pode declarar submódulos. Por exemplo, você pode declarar mod vegetables;em src/garden.rs . O compilador procurará o código do submódulo dentro do diretório nomeado para o módulo pai nestes locais:
// ° Em linha, seguindo diretamente mod vegetables, dentro de chaves em vez do ponto e vírgula
// ° No arquivo src/garden/vegetables.rs
// ° No arquivo src/garden/vegetables/mod.rs

// Caminhos para o código em módulos: Depois que um módulo fizer parte da sua caixa, você poderá consultar o código desse módulo de qualquer outro lugar na mesma caixa, desde que as regras de privacidade permitam, usando o caminho para o código. Por exemplo, um Asparagustipo no módulo garden vegetables seria encontrado em crate::garden::vegetables::Asparagus.

// Privado vs. público: O código dentro de um módulo é privado de seus módulos pais por padrão. Para tornar um módulo público, declare-o com pub modem vez de mod. Para tornar os itens dentro de um módulo público também públicos, use pubantes de suas declarações.

// A use palavra-chave: Dentro de um escopo, a usepalavra-chave cria atalhos para itens para reduzir a repetição de caminhos longos. Em qualquer escopo que possa se referir a crate::garden::vegetables::Asparagus, você pode criar um atalho com use crate::garden::vegetables::Asparagus;e, a partir daí, basta escrever Asparaguspara usar esse tipo no escopo.

// - exemplo -

// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs