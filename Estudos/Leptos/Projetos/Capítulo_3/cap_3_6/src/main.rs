use leptos::{
    mount::mount_to_body,
    prelude::*,
    ev::SubmitEvent,
    html
};

fn App() -> impl IntoView {
    // Example 1
    let (name_01, set_name_01) = signal(String::from("Controlled"));
    let email = RwSignal::new(String::new());
    let favorite_color = RwSignal::new(String::from("red"));
    let spam_me = RwSignal::new(true);

    // Example 2
    let (name_02, set_name_02) = signal(String::from("Uncontrolled"));
    let input_element: NodeRef<html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
    
        let value = input_element.get()
                                 .expect("<input> should be mounted")
                                 .value();
        
        set_name_02.set(value);
    };

    // Example 3
    let some_value = RwSignal::new(String::new());

    // Example 4
    let (value, set_value) = signal(0i32);

    view! {
        <div
            style="display: flex; flex-direction: column; align-items: center"
        >
            <h1>
                "Cap_3_6 Leptos"
            </h1>
            <div>
                <h2>
                    "Example 1"
                </h2>
                <label>
                    "Enter an your name: "
                    <input
                        type="text"

                        // on:input:target=move |ev| {
                        //     set_name.set(ev.target().value());
                        // }
                        // prop:value=name

                        bind:value=(name_01, set_name_01)
                        style="display: block"
                    />
                </label>
                <label
                    style="display: block; margin-top: 1em"
                >
                    "Enter an your email: "
                    <input
                        type="email"
                        style="display: block"
                        bind:value=email
                    />
                </label>
                <label>
                    "Please send me lots of spam email."
                    <input
                        type="checkbox"
                        bind:checked=spam_me
                    />
                </label>
                <fieldset
                    style="margin-top: 1em; width: 10em; display: flex; flex-direction: column; align-items: center"
                >
                    <legend>
                        "What's an your favorite color?"
                    </legend>
                    <label>
                        "Red"
                        <input
                            type="radio"
                            name="color"
                            value="red"
                            bind:group=favorite_color
                        />
                    </label>
                    <label>
                        "Green"
                        <input
                            type="radio"
                            name="color"
                            value="green"
                            bind:group=favorite_color
                        />
                    </label>
                    <label>
                        "Blue"
                        <input
                            type="radio"
                            name="color"
                            value="blue"
                            bind:group=favorite_color
                        />
                    </label>
                </fieldset>
                <div>
                    <h3>
                        "Results"
                    </h3>
                    <p>
                        "Your favorite color is: " {favorite_color}
                    </p>
                    <p>
                        "Name is: " {name_01}
                    </p>
                    <p>
                        "Email is: " {email}
                    </p>
                    <Show
                        when=move || spam_me.get()
                    >
                        <p>
                            "You'll receive cool bonus content!"
                        </p>
                    </Show>
                </div>
            </div>
            <div>
                <h2>
                    "Example 2"
                </h2>
                <form
                    on:submit=on_submit
                >
                    <input 
                        type="text"
                        value=name_02
                        node_ref=input_element
                    />
                    <input 
                        type="submit"
                        value="Submit"
                    />
                </form>
                <div>
                    <h3>
                        "Result"
                    </h3>
                    <p>
                        "Name is: " {name_02}
                    </p>
                </div>
            </div>
            <div>
                <h2>
                    "Example 3"
                </h2>
                <textarea
                    prop:value=move || some_value.get()
                    on:input:target=move |ev| some_value.set(ev.target().value())
                >
                    {some_value}
                </textarea>
                <div>
                    <h3>
                        "Result"
                    </h3>
                    <p>
                        "Write is: " {some_value}
                    </p>
                </div>
            </div>
            <div>
                <h2>
                    "Example 4"
                </h2>
                <select
                    on:change:target=move |ev| {
                        set_value.set(ev.target().value().parse().unwrap());
                    }
                    prop:value=move || value.get().to_string()
                >
                    <option
                        value="0"
                    >
                        "0"
                    </option>
                    <option
                        value="1"
                    >
                        "1"
                    </option>
                    <option
                        value="2"
                    >
                        "2"
                    </option>
                </select>
                <button
                    on:click=move |_| set_value.update(|n| {
                        if *n == 2 {
                            *n = 0;
                        } else {
                            *n += 1;
                        }
                    })
                >
                    "Next Option"
                </button>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
