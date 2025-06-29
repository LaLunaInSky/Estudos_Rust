use leptos::{
    mount::mount_to_body,
    prelude::*
};

#[component]
fn StaticList(
    length: usize,
) -> impl IntoView {
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
        <ul>
            {counter_buttons}
        </ul>
    }
}

#[component]
fn DynamicList(
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);

        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });

        next_counter_id += 1;
    };

    view! {
        <div>
            <button
                on:click=add_counter
            >
                "Add Counter"
            </button>
            <ul>
                <For 
                    each=move || counters.get()

                    key=|counter| counter.0

                    children=move |(id, count)| {
                        let count = RwSignal::from(count);

                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

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
                {values.into_iter()
                    .map(|n| view! {
                        <li>
                            {n}
                        </li>
                    })
                    // .collect::<Vec<_>>()}
                    .collect_view()}
            </ul>
        </div>
        <h2>
            "Iteration"
        </h2>
        <h3>
            "Static List"
        </h3>
        <p>
            "Use this pattern if the list itself is static."
        </p>
        <StaticList length=5/>
        <h3>
            "Dynamic List"
        </h3>
        <p>
            "Use this pattern if the rows in your list will change."
        </p>
        <DynamicList initial_length=5/>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}