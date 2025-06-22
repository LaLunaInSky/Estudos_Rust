use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    view! {
        <h1>
            "Cap_3_6 Leptos"
        </h1>
    }
}

fn main() {
    mount_to_body(App);
}
