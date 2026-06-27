use leptos::{attr::Attribute, prelude::*};

use crate::icons;

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

#[component]
fn SidebarHeader(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex items-center px-2 py-4">
            <a href="/" class="flex items-center space-x-2">
                <img src="/assets/logo.svg" alt="iDesoft Logo" class="h-7 w-auto" />
                <span class="text-base font-bold text-blue-950 dark:text-white">"F5A Leptos"</span>
            </a>

            <button
                class="lg:hidden ml-auto p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
                on:click=move |_| is_sidebar_open.set(false)
            >
                <span class="sr-only">"Close sidebar"</span>
                <icons::XMark />
            </button>
        </div>
    }
}

#[component]
fn SidebarNav() -> impl IntoView {
    view! {
        <nav class="flex flex-col space-y-0.5 overflow-y-auto">
            <NavItem label="Home">
                <icons::Home />
            </NavItem>
            <NavItem label="Users">
                <icons::Users />
            </NavItem>
        </nav>
    }
}

#[component]
fn NavItem(children: Children, label: &'static str) -> impl IntoView {
    view! {
        <a class="flex items-center space-x-2 px-2 py-2 rounded-md hover:bg-slate-50 dark:hover:bg-slate-700">
            <div class="text-slate-400 dark:text-slate-500">
                {children()}
            </div>
            <span class="text-sm text-slate-600 font-medium dark:text-slate-300">{label}</span>
        </a>
    }
}

#[component]
fn SidebarOverlay(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    let overlay_classes = move || {
        if is_sidebar_open.get() {
            "fixed inset-0 z-40 bg-slate-900/60 backdrop-blur-sm transition-opacity lg:hidden"
        } else {
            "hidden"
        }
    };
    view! {
        <div
            class=overlay_classes
            on:click=move |_| is_sidebar_open.set(false)
        ></div>
    }
}

fn aside_class_attrs(is_sidebar_open: RwSignal<bool>) -> impl Attribute {
    let base_classes = "fixed inset-y-0 left-0 z-50 w-72 flex flex-none flex-col transition-transform duration-300 transform bg-white border-r border-blue-100 px-2 py-1 lg:translate-x-0 lg:static lg:inset-0 shadow-xl lg:shadow-none dark:bg-slate-800 dark:border-slate-700";

    view! {
        <{..} class=move || {
                if is_sidebar_open.get() {
                    format!("{} translate-x-0", base_classes)
                } else {
                    format!("{} -translate-x-full", base_classes)
                }
            } />
    }
}
