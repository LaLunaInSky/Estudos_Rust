use std::process::Command;
use aggregator::{SocialPost, NewsArticle, Summary, notify};

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!(
        "1 new social post: {}", post.summarize()
    );

    //
    println!();

    let article = NewsArticle {
        headline: String::from(
            "Penguins wun the Stanley Cup Championship!",
        ),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey tem in the NHL.",
        ),
    };

    println!(
        "New article available! {}", article.summarize()
    );

    //
    println!();

    notify(&post);
    notify(&article);
}