use leptos::{attr::Attribute, prelude::*};

use crate::icons;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="w-full flex flex-row justify-between px-4 py-4 border-b border-blue-100 bg-white">
            <div class="flex">
                <button {..clickable_icon_styles_attrs()}>
                    <span class="sr-only">Open sidebar</span>
                    <icons::Bars3 />
                </button>
            </div>

            <div class="flex">
                <button {..clickable_icon_styles_attrs()}>
                    <icons::Moon />
                </button>
            </div>
        </div>
    }
}

fn clickable_icon_styles_attrs() -> impl Attribute {
    view! {
        <{..} class="rounded-md p-1 text-slate-400 hover:bg-slate-200" />
    }
}
