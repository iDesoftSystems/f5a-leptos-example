use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::layout::{Navbar, Sidebar, SidebarContext};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    provide_context(SidebarContext(RwSignal::new(false)));

    view! {
        <div class="h-screen flex flex-row bg-gray-50 dark:bg-slate-900">
            <Sidebar />

            <div class="flex-1 flex flex-col">
                <Navbar />

                <main class="h-screen w-full p-4">
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
