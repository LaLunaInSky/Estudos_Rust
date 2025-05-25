use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let team_blue = String::from("Blue");
    let team_yellow = String::from("Yellow");

    let score_blue = scores.get(&team_blue).copied().unwrap_or(0);
    let score_yellow = scores.get(&team_yellow).copied().unwrap_or(0);

    println!(
        "The team {team_blue} have score to {score_blue}"
    );
    println!(
        "The team {team_yellow} have score to {score_yellow}"
    );

    // or
    println!();

    for (key, value) in &scores {
        println!(
            "The team {key} have score to {value}"
        );
    }

    //
    let field_name_01 = String::from("Favorite color");
    let field_value_01 = String::from("Blue");

    let mut map_01 = HashMap::new();
    map_01.insert(
        &field_name_01, &field_value_01
    );

    println!(
        "\n{map_01:?}\n{field_name_01}\n{field_value_01}"
    );

    //
    scores.insert(
        team_blue, 25
    );

    println!(
        "\n{scores:?}"
    );

    //
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!(
        "\n{scores:?}"
    );

    //
    let text_01 = "hello world wonderful world";

    let mut map_02 = HashMap::new();

    for word in text_01.split_whitespace() {
        let count = map_02.entry(word).or_insert(0);
        *count += 1;
    }

    println!(
        "\n{map_02:?}"
    );
}