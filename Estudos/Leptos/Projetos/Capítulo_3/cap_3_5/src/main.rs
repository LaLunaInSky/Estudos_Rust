use leptos::{
    mount::mount_to_body,
    prelude::*
};

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
fn App() -> impl IntoView {
    let (data, _set_data) = signal(vec![
        DatabaseEntry {
            key: String::from("foo"),
            value: RwSignal::new(10),
        },
        DatabaseEntry {
            key: String::from("bar"),
            value: RwSignal::new(20),
        },
        DatabaseEntry {
            key: String::from("baz"),
            value: RwSignal::new(15),
        },
    ]);
    
    view! {
        <h1>
            "Cap_3_5 Leptos"
        </h1>
        <button
            on:click=move |_| {
                // set_data.update(|data| {
                //     for row in data {
                //         row.value *= 2;
                //     }
                // });

                for row in &*data.read() {
                    row.value.update(|value| *value *= 2);
                }
            
                leptos::logging::log!("{:?}", data.get());
            }
        >
            "Update Values"
        </button>
        <For
            each=move || data.get()
            key=|state| state.key.clone()
            let(child)
        >
            <p>
                {child.value}
            </p>
        </For>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}