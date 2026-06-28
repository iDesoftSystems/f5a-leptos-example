use leptos::prelude::*;

use crate::layout::{MainContent, Sidebar};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    let is_sidebar_open = RwSignal::new(true);

    view! {
        <div class="h-screen flex flex-row bg-gray-50 dark:bg-slate-900">
            <Sidebar is_sidebar_open=is_sidebar_open />

            <MainContent is_sidebar_open=is_sidebar_open>
            {children()}
            </MainContent>
        </div>

    }
}
