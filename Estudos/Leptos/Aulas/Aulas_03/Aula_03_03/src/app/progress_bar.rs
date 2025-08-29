use leptos::prelude::*;

#[component]
pub fn ProgressBar(
    #[prop(default = 50)]
    max_limit: u16,
    
    #[prop(into)]
    value_progess: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max_limit
            value=value_progess
        />
    }
}