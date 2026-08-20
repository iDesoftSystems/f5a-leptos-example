use crate::features::users::models::UserItem;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::params::{Params, ParamsMap};

#[derive(Params, PartialEq, Clone)]
pub struct UserListParams {
    q: Option<String>,
    page: Option<usize>,
}

#[component]
pub fn UserListPage() -> impl IntoView {
    let (users, _users_tx) = signal(vec![
        UserItem {
            id: 1,
            name: "iDesoft Systems".to_string(),
            email: "info@idesoft.io".to_string(),
            username: "idesoftd".to_string(),
        },
        UserItem {
            id: 2,
            name: "Blue Bird".to_string(),
            email: "bluebird@idesoft.io".to_string(),
            username: "bluebird".to_string(),
        },
        UserItem {
            id: 2,
            name: "Portal Suite".to_string(),
            email: "portal@idesoft.io".to_string(),
            username: "portalsuite".to_string(),
        },
    ]);

    let query_params = leptos_router::hooks::use_query::<UserListParams>();

    let search_name: Memo<String> = Memo::new(move |_| {
        query_params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.q.as_deref())
            .unwrap_or_default()
            .to_owned()
    });

    let search_page = Memo::new(move |_| {
        query_params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.page)
            .unwrap_or_default()
    });

    let users_filtered = Memo::new(move |_| {
        let search_value = search_name.get();

        users.with(|items| {
            if search_value.is_empty() {
                return items.to_owned();
            }

            items
                .iter()
                .filter(|item| {
                    item.name
                        .to_lowercase()
                        .contains(&search_value.to_lowercase())
                })
                .cloned()
                .collect()
        })
    });

    let on_go_to_create = move |_| {
        leptos_router::hooks::use_navigate()("/users/create", Default::default());
    };

    let on_go_to_filtered_users = move |_| {
        let mut params = ParamsMap::new();
        params.insert("q", String::from("blue"));
        params.insert("page", 0.to_string());

        let path = format!("/users{}", params.to_query_string());

        leptos_router::hooks::use_navigate()(&path, Default::default());
    };

    view! {
        <div class="flex flex-row justify-between">
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">Users</h1>
            <button
                on:click=on_go_to_create
                class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                Create user
            </button>
            <button
                on:click=on_go_to_filtered_users
                class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                Filtered
            </button>
        </div>

        <ul class="list-disc list-inside">
            <For
                each=move || users_filtered.get()
                key=|item| item.id
                let(item)>
                <li class="text-base text-blue-950 dark:text-white">
                    <A href=format!("/users/{}", item.id) attr:class="hover:underline">{item.name}</A>
                </li>
            </For>
        </ul>

        <pre class="text-base text-blue-950 dark:text-white">
        "Page: "{search_page}
        </pre>
    }
}
