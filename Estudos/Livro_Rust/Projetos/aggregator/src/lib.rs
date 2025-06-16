use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> self {
        self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!(
                "The largest member is x = {}", self.x
            );
        } else {
            println!(
                "The largest member is y = {}", self.y
            );
        }
    }
}


pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!(
//         "Breaking news! {}", item.summarize()
//     );
// }

pub fn notify<T: Summary>(item: &T) {
    println!(
        "Breaking news! {}", item.summarize()
    );
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// pub fn notify<T: Summary>(item1: &T, item2: &T) {

// pub fn notify(item: &(impl Summary + Display)) {

// pub fn notify<T: Summary + Display>(item: &T) {

// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &T) -> i32 {

// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//      T: Display + Clone,
//      U: Clone + Debug,
// {

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    // fn summarize(&self) -> String {
    //     format!(
    //         "{}, by {} ({})", self.headline, self.author, self.location
    //     )
    // }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    // fn summarize(&self) -> String {
    //     format!(
    //         "{}: {}", self.username, self.content
    //     )
    // }
}

pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}