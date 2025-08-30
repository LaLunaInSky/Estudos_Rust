use dioxus::prelude::*;

#[component]
pub fn Title() -> Element {
    rsx! {
        div {
            class: "text-neutral-100 font-bold text-3xl py-2 text-center border-b-2 mb-2",
            h1 {
                "HotDog! 🌭"
            }
        }
    }
}