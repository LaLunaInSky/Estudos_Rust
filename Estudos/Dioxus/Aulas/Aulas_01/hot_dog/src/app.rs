use dioxus::prelude::*;

mod dog_view;
mod title;

use dog_view::DogView;
use title::Title;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { 
            rel: "icon", 
            href: FAVICON 
        }

        document::Link { 
            rel: "stylesheet", 
            href: TAILWIND_CSS 
        }

        body {
            class: "bg-black min-h-svh *:select-none",
            Title {},
            DogView {}
        }
    }
}