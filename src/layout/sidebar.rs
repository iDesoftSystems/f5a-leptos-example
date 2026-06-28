use leptos::{attr::Attribute, prelude::*};

use crate::{icons, theme::clickable_icon_styles_attrs};

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        // <SidebarOverlay />

        <aside {..aside_class_attrs()}>
            <SidebarHeader />
            <SidebarNav />
        </aside>
    }
}

fn aside_class_attrs() -> impl Attribute {
    let base_class = "flex flex-col w-72 px-2 py-4 border-r bg-white border-blue-100 gap-y-4 z-50 dark:bg-slate-800 dark:border-slate-700";
    let position_class = format!("fixed inset-y-0 left-0 lg:static {}", base_class);
    let full_class = format!("{}", position_class);

    view! {
        <{..} class=full_class />
    }
}

#[component]
fn SidebarOverlay() -> impl IntoView {
    view! {
        <div class="fixed inset-0 backdrop-blur-sm z-40">
        </div>
    }
}

#[component]
fn SidebarHeader() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center">
            <div class="flex gap-1 items-center">
                <img class="h-7 w-auto" src="/assets/logo.svg" alt="iDesoft Logo" />
                <span class="font-bold text-blue-950 text-base dark:text-white">F5A Leptos</span>
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
        <a class="flex flex-row items-center gap-x-2 py-2 px-0.5 hover:bg-slate-200  dark:hover:bg-slate-700 rounded-md">
            <div class="text-slate-400">
                {icon.run()}
            </div>
            <span class="text-slate-600 text-sm font-medium dark:text-slate-300">{label}</span>
        </a>
    }
}
