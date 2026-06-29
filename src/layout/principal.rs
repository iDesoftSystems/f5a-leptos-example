use leptos::prelude::*;
use leptos_use::ColorMode;

use crate::layout::{Navbar, Sidebar};

#[component]
pub fn Principal(
    children: Children,
    mode: Signal<ColorMode>,
    mode_tx: WriteSignal<ColorMode>,
) -> impl IntoView {
    let is_sidebar_open = RwSignal::new(true);

    view! {
        <div class="h-screen flex flex-row bg-gray-50 dark:bg-slate-900">
            <Sidebar is_sidebar_open=is_sidebar_open />

            <div class="flex-1 flex flex-col">
                <Navbar mode=mode mode_tx=mode_tx is_sidebar_open=is_sidebar_open />

                <main class="h-screen w-full p-4">
                {children()}
                </main>
            </div>
        </div>

    }
}
