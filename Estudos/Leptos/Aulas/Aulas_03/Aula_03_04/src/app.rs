use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let values = vec![0, 1, 2];

    let 

    view! {
        <h1>"Hello, World!"</h1>
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
                    // .collect::<Vec<_>>()
                    .collect_view()
                }
            </ul>
        </div>
    }
}