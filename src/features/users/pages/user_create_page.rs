use leptos::logging::log;
use leptos::prelude::*;

fn first_file_from(input: NodeRef<leptos::html::Input>) -> Option<web_sys::File> {
    input.get()?.files()?.get(0)
}

#[component]
pub fn UserCreatePage() -> impl IntoView {
    let file_ref = NodeRef::<leptos::html::Input>::new();

    let username = RwSignal::new(String::new());
    let gender = RwSignal::new(String::new());
    let terms_of_service = RwSignal::new(false);
    let has_file = RwSignal::new(false);

    let on_file_change = move |_| {
        has_file.set(first_file_from(file_ref).is_some());
    };

    let username_error = move || {
        let value = username.get();
        if value.trim().is_empty() {
            return Some("Username is required");
        }

        None
    };

    let gender_error = move || {
        let value = gender.get();

        if value.trim().is_empty() {
            return Some("Gender is required");
        }

        None
    };

    let terms_error = move || {
        let value = terms_of_service.get();
        if !value {
            return Some("You must accept the terms of service");
        }

        None
    };

    let file_error = move || {
        if !has_file.get() {
            return Some("At least one file is required");
        }

        None
    };

    let has_errors = Memo::new(move |_| {
        username_error().is_some()
            || gender_error().is_some()
            || terms_error().is_some()
            || file_error().is_some()
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let username = username.get();
        let gender = gender.get();
        let terms_of_service = terms_of_service.get();

        if let Some(file) = first_file_from(file_ref) {
            log!("file name: {}", file.name());
            log!("file size: {}", file.size());
            log!("file type: {}", file.type_());
        }

        log!("username: {}", username);
        log!("gender selected: {}", gender);
        log!("terms_of_service selected: {}", terms_of_service);
    };

    view! {
        <div class="mx-auto w-2xl max-w-2xl">

            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">User Create</h1>

            <form on:submit=on_submit class="flex flex-col gap-y-4">

                <div class="flex flex-row gap-x-2 w-full">
                    <div class="flex flex-col gap-y-2 w-full">
                        <label class="text-blue-950 dark:text-slate-300" for="username">Username</label>
                        <input
                            id="username"
                            bind:value=username
                            autocomplete="username"
                            class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                            type="text" />
                        <Show when=move || username_error().is_some()>
                            <p class="text-sm text-red-600">
                            {move || username_error().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="flex flex-col gap-y-2 w-full">
                        <label class="text-blue-950 dark:text-slate-300" for="gender">Gender</label>
                        <select
                            id="gender"
                            class="appearance-none rounded-md border border-blue-100 bg-white text-blue-950 py-2 px-1.5 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                            bind:value=gender>
                            <option value="" disabled selected>Select gender</option>
                            <option value="F">Female</option>
                            <option value="M">Male</option>
                            <option value="O">Other</option>
                        </select>
                        <Show when=move || gender_error().is_some()>
                            <p class="text-sm text-red-600">
                            {move || gender_error().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>
                </div>

                <div class="flex flex-col gap-y-2">
                    <input
                        node_ref=file_ref
                        on:change=on_file_change
                        id="avatar"
                        class="block w-full text-sm text-slate-500
                            file:mr-4 file:py-2 file:px-4
                            file:rounded-md file:border-0
                            file:text-sm file:font-semibold
                            file:bg-blue-50 file:text-blue-700
                            hover:file:bg-blue-100
                            cursor-pointer border border-blue-100 rounded-md
                            dark:border-slate-700 dark:file:bg-slate-50 dark:file:text-slate-700"
                        type="file" />
                    <Show when=move || file_error().is_some()>
                        <p class="text-sm text-red-600">
                        {move || file_error().unwrap_or_default()}
                        </p>
                    </Show>
                </div>

                <div class="flex flex-col gap-x-2">
                    <div class="flex flex-row gap-x-2">
                        <div class="flex flex-row gap-x-2">
                            <input
                                id="terms-of-service"
                                bind:checked=terms_of_service
                                class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                                type="checkbox" />
                            <label class="text-blue-950 dark:text-slate-300" for="terms-of-service">I agree to the terms of service</label>
                        </div>
                    </div>
                    <Show when=move || terms_error().is_some()>
                        <p class="text-sm text-red-600">
                        {move || terms_error().unwrap_or_default()}
                        </p>
                    </Show>
                </div>

                <button
                    type="submit"
                    disabled=move || has_errors.get()
                    class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800 disabled:bg-slate-400 disabled:cursor-not-allowed">
                    Save
                </button>
            </form>
        </div>
    }
}
