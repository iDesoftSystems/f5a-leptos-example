use leptos::prelude::*;

use crate::{icons, theme::clickable_icon_styles_attrs};

#[component]
pub fn Navbar(is_sidebar_open: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="w-full flex flex-row justify-between px-4 py-4 border-b border-blue-100 bg-white dark:bg-slate-800 dark:border-slate-700">
            <div class="flex">
                <button
                    on:click=move |_| is_sidebar_open.set(true)
                    {..clickable_icon_styles_attrs("lg:hidden")}>
                    <span class="sr-only">Open sidebar</span>
                    <icons::Bars3 />
                </button>
            </div>

            <div class="flex">
                <button {..clickable_icon_styles_attrs("")}>
                    <icons::Moon />
                </button>
            </div>
        </div>
    }
}
