use leptos::prelude::*;

use crate::{
    components::{
        CorrectCounter, DynamicList, InvalidCounter, Notifications, PasswordValidator,
        PasswordValidatorMemoized, SignalsDemo, StaticList, ThemeConfig, Toggler,
    },
    layouts,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <layouts::Principal>
            <h1 class="text-2xl font-extrabold text-blue-950">F5A Leptos</h1>
            <p>something here...</p>

            // <InvalidCounter />
            // <CorrectCounter />

            // <StaticList />
            // <DynamicList />

            // <Toggler />

            // <SignalsDemo />
            // <PasswordValidator />
            // <PasswordValidatorMemoized />
            <ThemeConfig />
            <Notifications />
        </layouts::Principal>
    }
}
