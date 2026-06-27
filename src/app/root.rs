use leptos::prelude::*;

use crate::layouts;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <layouts::Principal>
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">F5A Leptos</h1>
            <p class="dark:text-slate-300">something here...</p>
        </layouts::Principal>
    }
}
