use leptos::{
    mount::mount_to_body,
    prelude::*
};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: String::from("foo"),
            value: 10,
        },
        DatabaseEntry {
            key: String::from("bar"),
            value: 20,
        },
        DatabaseEntry {
            key: String::from("baz"),
            value: 15,
        },
    ]);
    
    view! {
        <h1>
            "Cap_3_5 Leptos"
        </h1>
        <button
            on:click=move |_| {
                set_data.update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            
                leptos::logging::log!("{:?}", data.get());
            }
        >
            "Update Values"
        </button>
        <For
            each=move || data.get()
            key=|state| (state.key.clone(), state.value)
            let(child)
        >
            <p>
                {child.value}
            </p>
        </For>
    }
}

fn main() {
    mount_to_body(App);
}