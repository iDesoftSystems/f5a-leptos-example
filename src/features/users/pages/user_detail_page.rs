use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Params, PartialEq, Clone, Default)]
pub struct UserDetailParams {
    pub user_id: i32,
}

#[component]
pub fn UserDetailPage() -> impl IntoView {
    let params = use_params::<UserDetailParams>();
    let user_id = Memo::new(move |_| params.read().as_ref().ok().map(|p| p.user_id));

    view! {
        <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">User Detail</h1>

        <pre class="text-base text-blue-950 dark:text-white">userId: { user_id }</pre>
    }
}
