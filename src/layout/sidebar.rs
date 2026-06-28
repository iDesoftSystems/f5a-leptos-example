use leptos::prelude::*;

use crate::{icons, theme::clickable_icon_styles_attrs};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="flex flex-col w-72 px-2 py-4 border-r border-blue-100 gap-y-4">
            <SidebarHeader />
            <SidebarNav />
        </aside>
    }
}

#[component]
fn SidebarHeader() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center">
            <div class="flex gap-1 items-center">
                <img class="h-7 w-auto" src="/assets/logo.svg" alt="iDesoft Logo" />
                <span class="font-bold text-blue-950 text-base">F5A Leptos</span>
            </div>

            <button {..clickable_icon_styles_attrs()}>
                <icons::XMark />
            </button>
        </div>
    }
}

#[component]
fn SidebarNav() -> impl IntoView {
    view! {
        <nav class="flex flex-col">
            <NavItem label="Home" icon=|| view! {<icons::Home />} />
            <NavItem label="Users" icon=|| view! {<icons::Users />} />
        </nav>
    }
}

#[component]
fn NavItem(label: &'static str, #[prop(into)] icon: ViewFn) -> impl IntoView {
    view! {
        <a class="flex flex-row items-center gap-x-2 py-2 px-0.5 hover:bg-slate-200 rounded-md">
            <div class="text-slate-400">
                {icon.run()}
            </div>
            <span class="text-slate-600 text-sm font-medium">{label}</span>
        </a>
    }
}
