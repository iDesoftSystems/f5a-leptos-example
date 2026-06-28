use leptos::prelude::*;

use crate::layout::MainContent;

#[component]
pub fn Principal(children: Children) -> impl IntoView {
    view! {
        <MainContent>
        {children()}
        </MainContent>
    }
}
