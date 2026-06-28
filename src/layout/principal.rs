use leptos::prelude::*;

use crate::layout::{MainContent, Sidebar};

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    view! {
        <div class="h-screen flex flex-row">
            <Sidebar />

            <MainContent>
            {children()}
            </MainContent>
        </div>

    }
}
