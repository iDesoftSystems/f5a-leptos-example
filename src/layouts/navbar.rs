use leptos::{attr::Attribute, prelude::*};
use leptos_use::ColorMode;

use crate::icons;

fn icon_style_attrs() -> impl Attribute {
    view! {
        <{..} class="w-6 h-6 text-slate-400" />
    }
}

#[component]
pub fn Navbar(
    is_sidebar_open: RwSignal<bool>,
    mode: Signal<ColorMode>,
    set_mode: WriteSignal<ColorMode>,
) -> impl IntoView {
    view! {
        <header class="flex justify-between items-center px-4 py-4 bg-white border-b border-blue-100 dark:bg-slate-800 dark:border-slate-700">
            <div class="flex items-center">
                <button class="rounded-md hover:bg-slate-100 dark:hover:bg-slate-700 h-full lg:hidden"
                    on:click=move |_| is_sidebar_open.set(true)
                    >
                    <span class="sr-only">"Open sidebar"</span>
                    <icons::Bars3 {..icon_style_attrs()} />
                </button>
            </div>

            <div class="flex items-center gap-2">
                <ToggleTheme mode set_mode />
                <UserAvatar />
            </div>

        </header>
    }
}

#[component]
fn UserAvatar() -> impl IntoView {
    view! {
        <button class="h-8 w-full flex flex-row items-center space-x-1 rounded-md text-sm text-blue-950 font-medium bg-slate-50 px-2 hover:bg-slate-100 dark:bg-slate-700 dark:text-blue-100 dark:hover:bg-slate-600 transition-transform">
            <icons::User {..icon_style_attrs()} />
            <span>"idesoftd"</span>
        </button>
    }
}

#[component]
fn ToggleTheme(mode: Signal<ColorMode>, set_mode: WriteSignal<ColorMode>) -> impl IntoView {
    let toggle_theme = move |_| {
        let next = match mode.get() {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::Light,
            _ => ColorMode::Light,
        };
        set_mode.set(next);
    };

    view! {
        <button
            class="flex items-center justify-center h-8 w-8 p-1 rounded-md text-slate-600 dark:text-slate-300 hover:bg-gray-100 dark:hover:bg-slate-700 transition-colors"
            on:click=toggle_theme
            aria-label={move || match mode.get() {
                ColorMode::Light => "Modo oscuro",
                _ => "Modo claro",
            }}
        >
            {move || match mode.get() {
                ColorMode::Dark => view! { <icons::Sun /> }.into_any(),
                _ => view! { <icons::Moon /> }.into_any(),
            }}
        </button>
    }
}
