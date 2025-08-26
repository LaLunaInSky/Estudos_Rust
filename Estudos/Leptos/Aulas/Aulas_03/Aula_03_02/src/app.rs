use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    
    let double_count = move || count.get() * 2;
    
    let html = "<p>This is HTML will be injected.</p>";

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }

            //class:red=move || count.get() % 2 == 1            
            //class=("button-20", move || count.get() % 2 == 1)
            class=(["button-20", "rounded"], move || count.get() % 2 == 1)

            style="position: absolute"
            style:left=move || format!(
                "{}px",
                count.get() + 100
            )
            style:background-color=move || format!(
                "rgb({}, {}, 100)",
                count.get(), 
                100
            )
            style:max-width="400px"
            style=("--columns", move || count.get().to_string())
        >
            "Click to move! "
            {count}
        </button>
        <br/>
        <progress
            max="50"
            value=double_count
        />
        <p>
            "Double count: "
            {double_count}
        </p>
        <div inner_html=html/>
    }
}