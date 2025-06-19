use leptos::{
    mount::mount_to_body,
    prelude::*
};

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
        >
            "Click me: "
            {count}
        </button>
        <p>
            <strong>"Reactive: "</strong>
            {move || count.get()}
        </p>
        <p>
            <strong>"Reactive shorthand: "</strong>
            {count}
        </p>
        <p>
            <strong>"Not reactive: "</strong>
            {count.get()}
        </p>
    }
}

fn main() {
    mount_to_body(App);
}