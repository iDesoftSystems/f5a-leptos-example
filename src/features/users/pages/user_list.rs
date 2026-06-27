use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn UsersPage() -> impl IntoView {
    let navigate = leptos_router::hooks::use_navigate();

    let on_go_to_details = move |_| {
        navigate("/users/1", Default::default());
    };

    view! {
        <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">Users</h1>

        <A href=format!("/users/1") attr:class="text-blue-950 dark:text-white">
            Nav to users details
        </A>

        <button on:click=on_go_to_details class="border border-slate-400 rounded-md px-2 py-1 text-blue-950 dark:text-white">
            Go to users details
        </button>
    }
}
