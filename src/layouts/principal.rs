use leptos::prelude::*;

use crate::layouts::{Navbar, Sidebar};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    let is_sidebar_open = RwSignal::new(false);

    view! {
        <div class="flex h-screen">
            <Sidebar is_sidebar_open=is_sidebar_open />

            <div class="flex-1 flex flex-col min-w-0">
                <Navbar is_sidebar_open=is_sidebar_open />

                <main class="flex-1 overflow-x-hidden overflow-y-auto p-4" >
                    {children()}
                </main>
            </div>

        </div>
    }
}
