use leptos::{logging::log, prelude::*};

#[component]
pub fn InvalidCounter() -> impl IntoView {
    let (logged_in, set_logged_in) = signal(false);

    let on_toggle = move |_| {
        log!("change logged in value");
        set_logged_in.update(|value| *value = !*value);
    };

    view! {
        <button class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white"
            on:click=on_toggle>
            "Toggle State [IF-ELSE]"
        </button>

        {
            if logged_in.get() {
                view! {<p>"ON"</p>}
            } else {
                view! {<p>"OFF"</p>}
            }
        }
    }
}

#[component]
pub fn CorrectCounter() -> impl IntoView {
    let (logged_in, set_logged_in) = signal(false);

    let on_toggle = move |_| {
        log!("change logged in value");
        set_logged_in.update(|value| *value = !*value);
    };

    view! {
        <button
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white"
            on:click=on_toggle>
            "Toggle State [Show]"
        </button>

        <Show
            when=move || logged_in.get()
            fallback=||view! {<p>"Please sign in."</p>}>
            <p>"Welcome back!"</p>
        </Show>
    }
}
