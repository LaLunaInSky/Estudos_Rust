use dioxus::prelude::*;

#[component]
pub fn Buttons() -> Element {
    rsx! {
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