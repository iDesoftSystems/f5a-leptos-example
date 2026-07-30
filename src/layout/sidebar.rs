use leptos::{attr::Attribute, prelude::*};
use leptos_router::components::A;

use crate::{icons, theme::clickable_icon_styles_attrs};

#[component]
pub fn Sidebar(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <SidebarOverlay is_sidebar_open=is_sidebar_open />

        <aside {..aside_class_attrs(is_sidebar_open)}>
            <SidebarHeader is_sidebar_open=is_sidebar_open />
            <SidebarNav />
        </aside>
    }
}

fn aside_class_attrs(is_sidebar_open: RwSignal<bool>) -> impl Attribute {
    let layout_class = "flex flex-col w-72 px-2 py-4 border-r bg-white border-blue-100 gap-y-4 z-50 dark:bg-slate-800 dark:border-slate-700";
    let position_class = format!(
        "fixed inset-y-0 left-0 lg:static {} lg:translate-x-0",
        layout_class
    );
    let full_class = format!(" {}", position_class);

    view! {
        <{..} class=move || {
            if is_sidebar_open.get() {
                format!("{} translate-x-0", full_class)
            } else {
                format!("{} -translate-x-full", full_class)
            }
        } />
    }
}

#[component]
fn SidebarOverlay(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class=move || {
            if is_sidebar_open.get() {
                "fixed inset-0 backdrop-blur-sm z-40 lg:hidden"
            } else {
                "hidden"
            }
        }>
        </div>
    }
}

#[component]
fn SidebarHeader(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex justify-between items-center">
            <div class="flex gap-1 items-center">
                <img class="h-7 w-auto" src="/assets/logo.svg" alt="iDesoft Logo" />
                <span class="font-bold text-blue-950 text-base dark:text-white">F5A Leptos</span>
            </div>

            <button
                on:click=move |_| is_sidebar_open.set(false)
                {..clickable_icon_styles_attrs("lg:hidden")}
                >
                <icons::XMark />
            </button>
        </div>
    }
}

#[component]
fn SidebarNav() -> impl IntoView {
    view! {
        <nav class="flex flex-col">
            <NavItem label="Home" href="/" icon=|| view! {<icons::Home />} />
            <NavItem label="Users" href="/users" icon=|| view! {<icons::Users />} />
        </nav>
    }
}

#[component]
fn NavItem(label: &'static str, href: &'static str, #[prop(into)] icon: ViewFn) -> impl IntoView {
    view! {
        <A href=href attr:class="flex flex-row items-center gap-x-2 py-2 px-0.5 hover:bg-slate-200  dark:hover:bg-slate-700 rounded-md">
            <div class="text-slate-400">
                {icon.run()}
            </div>
            <span class="text-slate-600 text-sm font-medium dark:text-slate-300">{label}</span>
        </A>
    }
}
