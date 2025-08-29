mod progress_bar;

use progress_bar::ProgressBar;

use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
        >
            "Add Count"
        </button>
        <div>
            <ProgressBar
                value_progess=count
            />
            <br />
            <ProgressBar 
                value_progess=Signal::derive(double_count)
            />
        </div>
    }
}