use leptos::logging::log;
use leptos::prelude::*;

use crate::core::forms::{FileItem, ShowRule, collect_file_items, required, use_form_field};

#[component]
pub fn UserCreatePage() -> impl IntoView {
    let submitted = RwSignal::new(false);

    let username = use_form_field(
        String::new(),
        required("Username is required"),
        submitted,
        ShowRule::TouchedOrSubmitted,
    );
    let gender = use_form_field(
        String::new(),
        required("Gender is required"),
        submitted,
        ShowRule::TouchedOrSubmitted,
    );
    let terms = use_form_field(
        false,
        required("You must accept the terms of service"),
        submitted,
        ShowRule::TouchedOrSubmitted,
    );
    let avatar = use_form_field(
        Vec::<FileItem>::new(),
        required("At least one file is required"),
        submitted,
        ShowRule::TouchedOrSubmitted,
    );

    let on_avatar_change = move |ev: web_sys::Event| {
        avatar.value.set(collect_file_items(&ev));
        avatar.touched.set(true);
    };

    let valid = Memo::new(move |_| {
        username.error.get().is_none()
            && gender.error.get().is_none()
            && terms.error.get().is_none()
            && avatar.error.get().is_none()
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        submitted.set(true);

        if !valid.get() {
            return;
        }

        let username = username.value.get();
        let gender = gender.value.get();
        let terms_of_service = terms.value.get();

        if let Some(file) = avatar.value.get().first() {
            log!("file name: {}", file.name);
            log!("file size: {}", file.size);
            log!("file type: {}", file.type_);
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
                            bind:value=username.value
                            on:blur=move |ev| username.mark_touched.run(ev)
                            autocomplete="username"
                            class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                            type="text" />
                        <Show when=move || username.show_error.get()>
                            <p class="text-sm text-red-600">
                            {move || username.error.get().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>

                    <div class="flex flex-col gap-y-2 w-full">
                        <label class="text-blue-950 dark:text-slate-300" for="gender">Gender</label>
                        <select
                            id="gender"
                            class="appearance-none rounded-md border border-blue-100 bg-white text-blue-950 py-2 px-1.5 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                            on:blur=move |ev| gender.mark_touched.run(ev)
                            bind:value=gender.value>
                            <option value="" disabled selected>Select gender</option>
                            <option value="F">Female</option>
                            <option value="M">Male</option>
                            <option value="O">Other</option>
                        </select>
                        <Show when=move || gender.show_error.get()>
                            <p class="text-sm text-red-600">
                            {move || gender.error.get().unwrap_or_default()}
                            </p>
                        </Show>
                    </div>
                </div>

                <div class="flex flex-col gap-y-2">
                    <input
                        on:change=on_avatar_change
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
                    <Show when=move || avatar.show_error.get()>
                        <p class="text-sm text-red-600">
                        {move || avatar.error.get().unwrap_or_default()}
                        </p>
                    </Show>
                </div>

                <div class="flex flex-col gap-x-2">
                    <div class="flex flex-row gap-x-2">
                        <div class="flex flex-row gap-x-2">
                            <input
                                id="terms-of-service"
                                bind:checked=terms.value
                                on:blur=move |ev| terms.mark_touched.run(ev)
                                class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950 dark:text-slate-300 dark:border-slate-700 dark:bg-slate-700"
                                type="checkbox" />
                            <label class="text-blue-950 dark:text-slate-300" for="terms-of-service">I agree to the terms of service</label>
                        </div>
                    </div>
                    <Show when=move || terms.show_error.get()>
                        <p class="text-sm text-red-600">
                        {move || terms.error.get().unwrap_or_default()}
                        </p>
                    </Show>
                </div>

                <button
                    type="submit"
                    disabled=move || !valid.get()
                    class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800 disabled:bg-slate-400 disabled:cursor-not-allowed">
                    Save
                </button>
            </form>
        </div>
    }
}
