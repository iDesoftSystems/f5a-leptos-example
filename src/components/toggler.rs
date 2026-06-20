use leptos::prelude::*;

#[component]
pub fn Toggler() -> impl IntoView {
    let (is_open, is_open_mut) = signal(false);

    let toggle = move |_| is_open_mut.set(!is_open.get());
    let reset = move |_| is_open_mut.set(false);

    view! {
        <hr />
        <h2>Toggler</h2>

        <Show
            when=move || is_open.get()
            fallback=||view! {<p>"CLOSED!"</p>}>
            <p>"OPEN!"</p>
        </Show>

        <button
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white"
            on:click=toggle>"Toggle"</button>
        <button
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white"
            on:click=reset>"Reset"</button>
    }
}
