use leptos::prelude::*;
use leptos_meta::Html;
use leptos_use::{UseColorModeOptions, UseColorModeReturn, use_color_mode_with_options};

use crate::layout;

#[component]
pub fn App() -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Html {..} class=move || mode.get().to_string()/>
        <layout::Principal mode=mode mode_tx=set_mode>
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">F5A Leptos</h1>
        </layout::Principal>
    }
}
