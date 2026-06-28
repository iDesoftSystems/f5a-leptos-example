use leptos::prelude::*;

use crate::layout::{MainContent, Sidebar};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen flex flex-row bg-gray-50 dark:bg-slate-900">
            <Sidebar />

            <MainContent>
            {children()}
            </MainContent>
        </div>

    }
}
