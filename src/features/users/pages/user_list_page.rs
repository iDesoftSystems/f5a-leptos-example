use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn UserListPage() -> impl IntoView {
    view! {
        <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">Users</h1>

        <A href="/users/create" attr:class="text-blue-500 hover:underline">Create User</A>
    }
}
