use leptos::prelude::*;

use crate::layout;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <layout::Principal>
            <div></div>
            // <h1 class="text-2xl font-extrabold text-blue-950">F5A Leptos</h1>
        </layout::Principal>
    }
}
