use std::io;

fn obter_a_escolha_do_usuário_sobre_a_temperatura_que_quer_converter() -> u32 {
    loop {
        println!("Qual Conversor você quer usar?\n [ 1 ] Celsius Para Fahrenheit\n [ 2 ] Fahrenheit para Celsius");

        let mut resposta_da_pergunta = String::new();

        io::stdin().read_line(&mut resposta_da_pergunta).expect("Falha ao ler a linha");

        let resposta_da_pergunta: u32 = match resposta_da_pergunta.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if resposta_da_pergunta == 1 || resposta_da_pergunta == 2 {
            return resposta_da_pergunta;
        }
    }

}

fn obter_temperatura_celsius_digitado_pelo_usuário() -> f32 {    
    loop {
        println!("\nInforme a temperatura em Celsius que você quer converter");

        let mut temperatura_celsius = String::new();

        io::stdin()
            .read_line(&mut temperatura_celsius)
            .expect("Falha ao ler a temperatura");

        let temperatura_celsius: f32 = match temperatura_celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temperatura_celsius;
    }
}

fn obter_temperatura_fahrenheit_digitado_pelo_usuário() -> f32 {    
    loop {
        println!("\nInforme a temperatura em Fahrenheit que você quer converter");

        let mut temperatura_fahrenheit = String::new();

        io::stdin()
            .read_line(&mut temperatura_fahrenheit)
            .expect("Falha ao ler a temperatura");

        let temperatura_fahrenheit: f32 = match temperatura_fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return temperatura_fahrenheit;
    }
}

fn converter_celsius_em_fahrenheit(temperatura_celsius: f32) -> f32 {
    return ((temperatura_celsius * 1.8) + 32.0).floor();
}

fn converter_fahrenheit_em_celsius(temperatura_fahrenheit: f32) -> f32 {
    return ((temperatura_fahrenheit - 32.0) * 0.555555556).floor();
}

fn main() {
    let _convertor_escolhido: u32 = obter_a_escolha_do_usuário_sobre_a_temperatura_que_quer_converter();

    if _convertor_escolhido == 1 {
        let _temperatura_em_celsius_digitado_pelo_usuário: f32 = obter_temperatura_celsius_digitado_pelo_usuário();

        let temparuta_convertida_em_fahrenheit: f32 = converter_celsius_em_fahrenheit(_temperatura_em_celsius_digitado_pelo_usuário);

        println!("A temperatura de {_temperatura_em_celsius_digitado_pelo_usuário}°C, convertido em Farenheit é de {temparuta_convertida_em_fahrenheit}°F");
    } else {
        let _temperatura_em_fahrenheit_digitado_pelo_usuário: f32 = obter_temperatura_fahrenheit_digitado_pelo_usuário();

        let temperatura_convertida_em_celsius: f32 = converter_fahrenheit_em_celsius(_temperatura_em_fahrenheit_digitado_pelo_usuário);

        println!("A temperatura de {_temperatura_em_fahrenheit_digitado_pelo_usuário}°F, convertido em Celsius é de {temperatura_convertida_em_celsius}°C");
    }
}