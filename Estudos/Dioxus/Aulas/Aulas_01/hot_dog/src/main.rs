use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { 
            rel: "icon", 
            href: FAVICON 
        }

        document::Link { 
            rel: "stylesheet", 
            href: TAILWIND_CSS 
        }
        
        div {
            class: "text-red-500",
            h1 {
                "HotDog! 🌭"
            }
        }

        div {
            img {
                src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg"
            }
        }

        div {
            button {
                id: "skip",
                "skip"
            },
            button {
                id: "save",
                "save!"
            }
        }
    }
}
