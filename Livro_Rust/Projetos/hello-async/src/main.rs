use std::process::Command;
use trpl::Html;

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

fn clean_terminal() {
    Command::new("clear").status().unwrap();
}

fn main() {
    clean_terminal();

    println!("Hello, world!");
}