use leptos::prelude::*;

#[derive(Clone, Debug)]
struct Profile {
    name: String,
    age: u32,
}

#[component]
pub fn SignalsDemo() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let (is_active, set_active) = signal(false);
    let (text, set_text) = signal(String::new());
    let (profile, set_profile) = signal(Profile {
        name: "Alice".into(),
        age: 30,
    });

    // --- Contador: set vs update ---
    let inc_set = move |_| set_count.set(count.get() + 1);
    let inc_update = move |_| set_count.update(|n| *n += 1);
    let reset = move |_| set_count.set(0);

    // --- Toggle: set vs update ---
    let toggle_set = move |_| set_active.set(!is_active.get());
    let toggle_update = move |_| set_active.update(|v| *v = !*v);

    // --- String: update (push/clear) vs set (replace) ---
    let push_a = move |_| set_text.update(|s| s.push('a'));
    let push_b = move |_| set_text.update(|s| s.push('b'));
    let clear_text = move |_| set_text.set(String::new());

    // --- Struct: update vs set ---
    let birthday_set = move |_| {
        set_profile.set(Profile {
            age: profile.get().age + 1,
            ..profile.get()
        });
    };
    let birthday_update = move |_| set_profile.update(|p| p.age += 1);
    let rename = move |_| set_profile.update(|p| p.name = "Bob".into());

    view! {
        <hr/>
        <h2 class="text-xl font-bold mt-4">Señales: set vs update</h2>

        // --- Contador ---
        <h3 class="font-semibold mt-2">Contador</h3>
        <p>Valor: {count}</p>
        <button
            on:click=inc_set
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "+1 con set"
        </button>
        <button
            on:click=inc_update
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "+1 con update"
        </button>
        <button
            on:click=reset
            class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Reset"
        </button>

        // --- Toggle ---
        <h3 class="font-semibold mt-2">Toggle</h3>
        <Show
            when=move || is_active.get()
            fallback=|| view! { <p>"Inactivo"</p> }>
            <p>"Activo"</p>
        </Show>
        <button on:click=toggle_set class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Toggle con set"
        </button>
        <button on:click=toggle_update class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Toggle con update"
        </button>

        // --- String ---
        <h3 class="font-semibold mt-2">String builder</h3>
        <p>Texto: {text}</p>
        <button on:click=push_a class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Push 'a' (update)"
        </button>
        <button on:click=push_b class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Push 'b' (update)"
        </button>
        <button on:click=clear_text class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Clear (set)"
        </button>

        // --- Struct ---
        <h3 class="font-semibold mt-2">Struct Profile</h3>
        <p>{move || format!("{} — {} años", profile.get().name, profile.get().age)}</p>
        <button on:click=birthday_set class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Cumpleaños con set"
        </button>
        <button on:click=birthday_update class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Cumpleaños con update"
        </button>
        <button on:click=rename class="border rounded px-2 py-1 bg-white hover:bg-slate-700 hover:text-white">
            "Renombrar a Bob (update)"
        </button>
    }
}
