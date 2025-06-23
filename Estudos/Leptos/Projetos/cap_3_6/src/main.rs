use leptos::{
    mount::mount_to_body,
    prelude::*
};

fn App() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let email = RwSignal::new(String::new());
    let favorite_color = RwSignal::new(String::from("red"));
    let spam_me = RwSignal::new(true);

    view! {
        <div
            style="display: flex; flex-direction: column; align-items: center"
        >
            <h1>
                "Cap_3_6 Leptos"
            </h1>
            <div>
                <label>
                    "Enter an your name: "
                    <input
                        type="text"
                        
                        // on:input:target=move |ev| {
                        //     set_name.set(ev.target().value());
                        // }
                        // prop:value=name

                        bind:value=(name, set_name)
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
                    <h2>
                        "Results"
                    </h2>
                    <p>
                        "Your favorite color is: " {favorite_color}
                    </p>
                    <p>
                        "Name is: " {name}
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
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
