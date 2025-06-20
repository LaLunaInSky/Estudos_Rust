use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    let values = vec![0, 1, 2];
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));

    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <h1>
            Cap_3_4 Leptos
        </h1>
        <div>
            <p>
                {values.clone()}
            </p>
            <ul>
                {values.into_iter()
                    .map(|n| view! {
                        <li>
                            {n}
                        </li>
                    })
                    // .collect::<Vec<_>>()}
                    .collect_view()}
            </ul>
            <ul>
                {counter_buttons}
            </ul>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}