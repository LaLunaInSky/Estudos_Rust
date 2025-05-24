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
}