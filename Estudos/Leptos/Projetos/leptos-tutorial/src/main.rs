use leptos::{
    mount::mount_to_body,
    prelude::*
};

/// Shows progress toward a goal.
#[component]
fn ProgressBar(

    /// The maximun value of the progress bar.
    #[prop(default = 100)]
    max: u16,

    /// How much progress should be displayed.
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <div
            style="
                display: flex; 
                flex-direction: row; 
                align-items: center; 
                gap: 1em
            "
        >
            <progress
                max=max
                value=progress
            />
            <p>
                {progress}
            </p>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }

            class:red=move || count.get() % 2 == 1

            class=("button-20", move || count.get() % 2 == 0)

            style="position: absolute"

            style:left=move || format!(
                "{}px", count.get() + 100
            )

            style:background-color=move || format!(
                "rgb({}, {}, 100)", count.get(), 100
            )

            style:max-width="400px"

            style=("--columns", move || count.get().to_string())
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

        // <progress
        //     max="50"
        //     value=count
        // />
        // <progress
        //     max="50"
        //     value=move || count.get() * 2
        //     style="display: block"
        // />
        // <progress
        //     max="50"
        //     value=double_count
        // />

        <ProgressBar 
            max=50 
            progress=count
        />
        <ProgressBar 
            progress=Signal::derive(double_count)
        />

        <p>
            "Double Count: "
            {double_count}
        </p>
        <div inner_html=html/>
    }
}

fn main() {
    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App);
}