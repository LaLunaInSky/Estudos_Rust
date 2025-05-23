fn main() {
    // forma de setar uma string vazia
    let str_01 = String::new();
    println!("{}", str_01);
    
    // formas de setar uma string
    println!();

    let data_01 = "initial contents";
    println!("{}", data_01);

    let str_02 = data_01.to_string();
    println!("{}", str_02);

    let str_03 = "initial contents".to_string();
    println!("{}", str_03);

    let str_04 = String::from("initial contents");
    println!("{}", str_04);

    // String é codificado em UTF-8, todos esses são valores abaixo são válidos.
    println!();

    let hellos: [String; 11] = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for hello in &hellos {
        println!("{}", hello);
    }

    // push_str (múltiplos chars) e push (único char)
    println!();

    let mut str_05 = String::from("foo");
    println!("{}", str_05);

    // str_05.push_str("bar");
    // println!("{}", str_05);

    let str_06 = "bar";

    str_05.push_str(str_06);
    println!("{}", str_05);

    println!("{}", str_06);

    let mut str_07 = String::from("lo");
    println!("{}", str_07);

    str_07.push('l');
    println!("{}", str_07); 

    // operador + (assume a propriedade do primeiro parametro) e format! (não assume a propriedade de nunhum parametro)
    println!();

    let str_08 = String::from("Hello, ");
    println!("{}", str_08);

    let str_09 = String::from("world!");
    println!("{}", str_09);

    let str_10 = str_08 + &str_09;
    println!("{}", str_10);

    println!();

    let str_11 = String::from("tic");
    println!("{}", str_11);

    let str_12 = String::from("tac");
    println!("{}", str_12);

    let str_13 = String::from("toe");
    println!("{}", str_13);

    // let str_14 = str_11 + "-" + &str_12 + "-" + &str_13;
    let str_14 = format!(
        "{str_11}-{str_12}-{str_13}"
    );
    println!("{}", str_14);

    // indexação em Strings
    println!();

    // código inválido a seguir! 
    // let str_15 = String::from("hi");
    // let h_01 = str_15[0];

    let hello_01 = "Здравствуйте";
    
    // outro código inválido!
    // let answer_01 = &hello_01[0];
    
    // Fatiação byte por byte
    // tomar cuidado ao fatiar assim, pois pode travar o programa!
    let answer_01 = &hello_01[0..4];

    println!("{answer_01:?}");

    // interação de string
    println!();

    for c in answer_01.chars() {
        println!("{c}");
    }

    println!();

    for b in answer_01.bytes() {
        println!("{b}");
    }
}