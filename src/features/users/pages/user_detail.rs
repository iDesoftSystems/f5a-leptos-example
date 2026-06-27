use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct UserDetailsParams {
    user_id: Option<i32>,
}

#[component]
pub fn UserDetailPage() -> impl IntoView {
    let route_params = use_params::<UserDetailsParams>();
    let user_id = Memo::new(move |_| {
        route_params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.user_id)
    });

    view! {
        <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">Users</h1>
        <code class="text-blue-950 dark:text-white">
            {user_id}
        </code>
    }
}
