fn main() {
    let mut str_01 = String::new();

    // mesmas funções para setar uma string
    let data_01 = "initial contents";

    let str_02 = data_01.to_string();

    let str_03 = "initial contents".to_string();

    let str_04 = String::from("initial contents");

    // String é codificado em UTF-8, todos esses são valores abaixo são válidos.
    let hello_01 = String::from("السلام عليكم");
    let hello_02 = String::from("Dobrý den");
    let hello_03 = String::from("hello");
    let hello_04 = String::from("שלום");
    let hello_05 = String::from("नमस्ते");
    let hello_06 = String::from("こんにちは");
    let hello_07 = String::from("안녕하세요");
    let hello_08 = String::from("你好");
    let hello_09 = String::from("Olá");
    let hello_10 = String::from("Здравствуйте");
    let hello_11 = String::from("Hola");
}