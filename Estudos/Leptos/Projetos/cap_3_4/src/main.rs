use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    let values = vec![0, 1, 2];

    view! {
        <h1>
            Cap_3_4 Leptos
        </h1>
        <div>
            <p>
                {values.clone()}
            </p>
            <ul>
                {value.into_iter()
                    .map(|n| view! {
                        <li>
                            {n}
                        </li>
                    })
                    .collect_view()}
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}