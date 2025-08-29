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

        body {
            class: "bg-black min-h-svh *:select-none",
            div {
                class: "text-neutral-100 font-bold text-3xl py-2 text-center border-b-2 mb-2",
                h1 {
                    "HotDog! 🌭"
                }
            },
            div {
                class: "flex justify-center items-center py-2",
                img {
                    class: "w-90 shadow-lg/70 shadow-neutral-100 rounded-lg ring-4 ring-neutral-100/70",
                    src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg"
                }
            },
            div {
                class: "*:text-neutral-100 flex justify-center items-center gap-4 my-3 *:py-2 *:font-bold *:text-2xl *:px-3 *:rounded-2xl *:hover:bg-neutral-100/80 *:cursor-pointer",
                button {
                    class: "bg-red-800 hover:text-red-800",
                    id: "skip",
                    "skip"
                },
                button {
                    class: "bg-green-800 hover:text-green-800",
                    id: "save",
                    "save!"
                }
            }
        }
    }
}
