use leptos::prelude::*;

mod static_list;
mod dinamic_list;

use static_list::StaticList;
use dinamic_list::DinamicList;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <h1>
            "Interation"
        </h1>
        <h2>
            "Static List"
        </h2>
        <p>
            "Use this pattern if the list itself is static."
        </p>
        <StaticList 
            length=5
        />
        <h2>
            "Dinamic List"
        </h2>
        <p>
            "Use this pattern if the rows in your list will change."
        </p>
        <DinamicList 
            initial_length=5
        />
    }
}