use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    view! {
        <div
            style="display: flex; flex-direction: column; align-items: center"
        >
            <h1>
                "Cap_3_7 Leptos"
            </h1>
            <div>
                <h2>
                    "Example 1"
                </h2>
                <p>
                    "The number " {value} " is "
                    {move || if is_odd() {
                        "Odd"
                    } else {
                        "Even"
                    }}
                </p>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}