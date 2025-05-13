use std::io;

fn obter_a_quantidade_de_termos_que_o_usuário_quer_ver() -> u32 {
    loop {
        println!("Quantos termos da sequência de Fibonacci você quer ver?");

        let mut quantidade_de_termos = String::new();

        io::stdin().read_line(&mut quantidade_de_termos).expect("Falha ao ler a linha");

        let _quantidade_de_termos: u32 = match quantidade_de_termos.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return _quantidade_de_termos;
    }
}

fn mostrar_n_termos_da_sequência_de_fibonacci(quantidade_de_termos: u32) {
    let mut posição_da_sequência: u32 = 1;
    let mut termo_da_sequência: u32 = 1;
    let mut antipenúltimo_termo_da_sequência: u32 = 0;
    let mut penúltimo_termo_da_sequência: u32 = 1;
    let mut último_termo_da_sequência: u32 = 1;

    println!();

    while termo_da_sequência <= quantidade_de_termos {
        if posição_da_sequência % 3 == 0 {
            print!("{último_termo_da_sequência} ");

            posição_da_sequência = 1;
            
            antipenúltimo_termo_da_sequência = último_termo_da_sequência + penúltimo_termo_da_sequência;

            penúltimo_termo_da_sequência = antipenúltimo_termo_da_sequência + último_termo_da_sequência;
            
            último_termo_da_sequência = antipenúltimo_termo_da_sequência + penúltimo_termo_da_sequência;

        } else if posição_da_sequência % 2 == 0 {
            print!("{penúltimo_termo_da_sequência} ");
        
            posição_da_sequência += 1;
        } else {
            print!("{antipenúltimo_termo_da_sequência} ");
            
            posição_da_sequência += 1;
        }

        termo_da_sequência += 1;
    }

    println!("\n");
}

fn main() {
    let _quantidade_de_termos_a_mostrar: u32 = obter_a_quantidade_de_termos_que_o_usuário_quer_ver();

    mostrar_n_termos_da_sequência_de_fibonacci(_quantidade_de_termos_a_mostrar);
}
