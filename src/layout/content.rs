use leptos::prelude::*;

use crate::layout::Navbar;

#[component]
pub fn MainContent(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <Navbar />

            <main class="h-4 w-full">
            {children()}
            </main>
        </div>
    }
}
