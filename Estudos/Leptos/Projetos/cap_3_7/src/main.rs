use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    // Example 1 with if
    let (value_01, _set_value_01) = signal(0);
    let is_odd_01 = move || value_01.get() % 2 != 0;

    // Example 2 with Option<T>
    let (value_02, _set_value_02) = signal(3);
    let is_odd_02 = move || value_02.get() % 2 != 0;

    let message_01 = move || {
        if is_odd_02() {
            Some("Ding ding ding!")
        } else {
            None
        }
    };

    let message_02 = move || is_odd_02().then(|| "Ding ding ding!");

    // Example 3 with match
    let (value_03, _set_value_03) = signal(3);
    let is_odd_03 = move || value_03.get() % 2 != 0;

    let message_03 = move || {
        match value_03.get() {
            0 => "Zero",
            1 => "One",
            n if is_odd_03() => "Odd",
            _ => "Even",
        }
    };

    // Example 4
    let (value_04, _set_value_04) = signal(2);
    let message_04 = move || if value_04.get() > 5 {
        // logging::log!("{}: rendering Big", value_04.get());
        "Big"
    } else {
        // logging::log!("{}: rendering Small", value_04.get());
        "Small"
    };

    // Example 5
    let (value_05, _set_value_05) = signal(0);

    view! {
        <div
            style="display: flex; flex-direction: column; align-items: center"
        >
            <h1>
                "Cap_3_7 Leptos"
            </h1>
            <div>
                <h2>
                    "Example 1 with if"
                </h2>
                <p
                    style="text-align: center"
                >
                    "The number " {value_01} " is "
                    {move || if is_odd_01() {
                        "Odd"
                    } else {
                        "Even"
                    }}
                </p>
            </div>
            <div>
                <h2>
                    "Example 2 with Option<T>"
                </h2>
                <p
                    style="text-align: center"
                >
                    {message_01}
                </p>
                <p
                    style="text-align: center"
                >
                    {message_02}
                </p>
            </div>
            <div>
                <h2>
                    "Example 3 with match"
                </h2>
                <p
                    style="text-align: center"
                >
                    {message_03}
                </p>
            </div>
            <div>
                <h2>
                    "Example 4"
                </h2>
                <p
                    style="text-align: center"
                >
                    {message_04}
                </p>
            </div>
            <div>
                <h2>
                    "Example 5"
                </h2>
                <Show
                    when=move || { value_05.get() > 5 }
                    fallback=move || view! { 
                        <p
                            style="text-align: center"
                        >
                            {value_05}: rendering Small
                        </p> 
                    }
                >
                    <p
                        style="text-align: center"
                    >
                        {value_05}: rendering Big
                    </p>
                </Show>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}