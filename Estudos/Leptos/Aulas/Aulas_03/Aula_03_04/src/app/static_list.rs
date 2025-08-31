use leptos::prelude::*;

#[component]
pub fn StaticList(
    length: usize
) -> impl IntoView {
    let counters = (1..=length).map(
        |idx| RwSignal::new(idx)
    );

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
    }).collect_view();
    
    view! {
        <ul>
            {counter_buttons}
        </ul>
    }
}