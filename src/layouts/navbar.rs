use leptos::{attr::Attribute, prelude::*};

use crate::icons;

fn icon_style_attrs() -> impl Attribute {
    view! {
        <{..} class="w-6 h-6 text-slate-400" />
    }
}

#[component]
pub fn Navbar(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <header class="flex justify-between items-center px-4 py-4 bg-white border-b border-blue-100">
            <div class="flex items-center">
                <button class="rounded-md hover:bg-slate-100 h-full lg:hidden"
                    on:click=move |_| is_sidebar_open.set(true)
                    >
                    <span class="sr-only">"Open sidebar"</span>
                    <icons::Bars3 {..icon_style_attrs()} />
                </button>
            </div>

            <div class="flex items-center">
                <UserAvatar />
            </div>

        </header>
    }
}

#[component]
fn UserAvatar() -> impl IntoView {
    view! {
        <button class="h-8 w-full flex flex-row items-center space-x-1 rounded-md text-sm text-blue-950 font-medium bg-slate-50 px-2 hover:bg-slate-100 transition-transform">
            <icons::User {..icon_style_attrs()} />
            <span>"idesoftd"</span>
        </button>
    }
}
