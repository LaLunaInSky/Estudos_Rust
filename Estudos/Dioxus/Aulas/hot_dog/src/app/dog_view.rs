use dioxus::prelude::*;

// mod dog_api;

// use dog_api::DogApi;

#[derive(serde::Deserialize)]
struct DogApi {
    message: String   
}

#[component]
pub fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
        .await
        .unwrap()
        .json::<DogApi>()
        .await
        .unwrap()
        .message
    });

    rsx! {
        div {
            class: "flex flex-col justify-center items-center pt-2",
            img {
                class: "w-90 shadow-lg/70 shadow-neutral-100 rounded-lg ring-4 ring-neutral-100/70 mb-2 mask-origin-content h-120 bg-neutral-50",
                src: (img_src.clone())().unwrap_or_default()
            },
            div {
                class: "*:text-neutral-100 flex justify-center items-center gap-4 my-3 *:py-2 *:font-bold *:text-2xl *:px-3 *:rounded-2xl *:hover:bg-neutral-100/80 *:cursor-pointer",
                button {
                    class: "bg-red-800 hover:text-red-800",
                    onclick: move |_| img_src.restart(),
                    "skip"
                },
                button {
                    class: "bg-green-800 hover:text-green-800",
                    onclick: move |_| img_src.restart(),
                    "save!"
                }
            }
        }
    }
}