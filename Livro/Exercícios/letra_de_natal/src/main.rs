fn main() {
    let nome_dos_números_em_extensivo_coleção: [&'static str; 12] = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", 
    "twelfth"];

    let nome_dos_números_em_extensivo: [&'static str; 12] = ["a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];

    let itens_recebidos: [&'static str; 12] = ["Partridge in a pear three.", "turtle doves,", "French Hens,", "calling birds,", "golden rings,", "geese a laying,", "swans a swimming,", "maids a milking,", "ladies dancing,", "lords a leaping,", "pipers piping,", "drummers drumming,"];

    println!();

    for count in 1..13 {

        println!("On the {} day of Christmas\nMy true love sent to me:", nome_dos_números_em_extensivo_coleção[count - 1]);

        for item in (0..count).rev() {
            if count == 1 {
                println!("{} {}", nome_dos_números_em_extensivo[item].to_uppercase(), itens_recebidos[item]);
            } else {
                if item == 0 {
                    println!("And {} {}", nome_dos_números_em_extensivo[item], itens_recebidos[item]);
                } else {
                    println!("{} {}", nome_dos_números_em_extensivo[item], itens_recebidos[item]);
                }
            }
            
        }

        println!();
    }
}