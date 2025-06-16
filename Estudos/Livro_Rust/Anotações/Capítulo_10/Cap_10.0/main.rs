use std::process::Command;

fn clean_of_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_of_terminal();

    let numbers_list_01 = vec![34, 50, 25, 100, 65];

    analisar_qual_é_o_número_maior_de_uma_lista(&numbers_list_01);

    let numbers_list_02 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    analisar_qual_é_o_número_maior_de_uma_lista(&numbers_list_02);

}

fn analisar_qual_é_o_número_maior_de_uma_lista(numbers_list: &[i32]) {
    let mut largest = numbers_list[0];
    
    for number in numbers_list {
        if *number > largest {
            largest = *number;
        }
    }
    
    println!("The largest number is {largest}");
}