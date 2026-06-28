use leptos::prelude::*;

use crate::layout::Navbar;

#[component]
pub fn MainContent(children: Children, is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex-1 flex flex-col">
            <Navbar is_sidebar_open=is_sidebar_open />

            <main class="h-screen w-full">
            {children()}
            </main>
        </div>
    }
}
