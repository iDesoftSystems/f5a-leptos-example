use leptos::prelude::*;
use leptos_meta::Html;
use leptos_router::components::Outlet;
use leptos_use::{ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::layouts::{Navbar, Sidebar};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    let is_sidebar_open = RwSignal::new(false);

    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().emit_auto(false));

    view! {
        <Html attr:class=move || {
            match mode.get() {
                ColorMode::Dark => "dark",
                _ => "",
            }
        } />

        <div class="flex h-screen bg-gray-50 dark:bg-slate-900">
            <Sidebar is_sidebar_open=is_sidebar_open />

            <div class="flex-1 flex flex-col min-w-0">
                <Navbar
                    is_sidebar_open=is_sidebar_open
                    mode=mode
                    set_mode=set_mode
                />

                <main class="flex-1 overflow-x-hidden overflow-y-auto p-4">
                    {children()}
                </main>
            </div>

        </div>
    }
}

#[component]
pub fn ProtectedRoutes() -> impl IntoView {
    view! {
        <Principal>
            <Outlet />
        </Principal>
    }
}
