use dioxus::prelude::*;

#[component]
pub fn DogView() -> Element {
    rsx! {
        div {
            class: "flex justify-center items-center py-2",
            img {
                class: "w-90 shadow-lg/70 shadow-neutral-100 rounded-lg ring-4 ring-neutral-100/70",
                src: "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg"
            }
        }
    }
}