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

    let on_go_to_create_post = move |_| {
        let user_id = user_id.get().unwrap();

        let path = format!("/users/{}/posts/create", user_id);
        leptos_router::hooks::use_navigate()(&path, Default::default());
    };

    view! {
        <div class="flex flex-row justify-between">
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">User Detail</h1>

            <button
                on:click=on_go_to_create_post
                class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                Create post
            </button>
        </div>

        <pre class="text-base text-blue-950 dark:text-white">userId: { user_id }</pre>
    }
}
