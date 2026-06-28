use leptos::prelude::*;

use crate::layout::Navbar;

#[component]
pub fn MainContent(children: Children) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col">
            <Navbar />

            <main class="h-screen w-full">
            {children()}
            </main>
        </div>
    }
}
