use leptos::prelude::*;
use leptos_use::ColorMode;

use crate::{icons, theme::clickable_icon_styles_attrs};

#[component]
pub fn Navbar(
    is_sidebar_open: RwSignal<bool>,
    mode: Signal<ColorMode>,
    mode_tx: WriteSignal<ColorMode>,
) -> impl IntoView {
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
                <ToggleTheme mode=mode mode_tx=mode_tx />
            </div>
        </div>
    }
}

#[component]
fn ToggleTheme(mode: Signal<ColorMode>, mode_tx: WriteSignal<ColorMode>) -> impl IntoView {
    let on_toggle_theme = move |_| {
        let next = match mode.get() {
            ColorMode::Light => ColorMode::Dark,
            ColorMode::Dark => ColorMode::Light,
            _ => ColorMode::default(),
        };

        mode_tx.set(next);
    };

    view! {
        <button
            {..clickable_icon_styles_attrs("")}
            on:click=on_toggle_theme
            >
            {move || {
                match mode.get() {
                    ColorMode::Dark => view! { <icons::Sun /> }.into_any(),
                    ColorMode::Light => view! { <icons::Moon /> }.into_any(),
                    _ => view! { <icons::Moon /> }.into_any()
                }
            }}
        </button>
    }
}
