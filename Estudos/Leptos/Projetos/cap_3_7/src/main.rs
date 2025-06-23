use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    let (value, set_value) = signal(0);
    let is_odd = move || value.get() % 2 != 0;

    view! {
        <h1>
            "Cap_3_7 Leptos"
        </h1>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}