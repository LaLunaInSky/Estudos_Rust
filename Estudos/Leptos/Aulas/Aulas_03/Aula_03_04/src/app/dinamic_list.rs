use leptos::prelude::*;

#[component]
pub fn DinamicList(
    initial_length: usize
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