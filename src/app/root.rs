use leptos::prelude::*;

use crate::layouts;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <layouts::Principal>
            <h1 class="text-2xl font-extrabold text-blue-950">F5A Leptos</h1>
            <p>something here...</p>
        </layouts::Principal>
    }
}
