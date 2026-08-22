use leptos::{logging::log, prelude::*};
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn LoginPage() -> impl IntoView {
    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form = ev
            .target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlFormElement>();

        let data = web_sys::FormData::new_with_form(&form).unwrap();
        let username = data.get("username").as_string().unwrap_or_default();
        let password = data.get("password").as_string().unwrap_or_default();

        log!("debug: username: {:?}, password: {:?}", username, password);
    };

    view! {
        <div class="h-screen w-full flex items-center justify-center">
            <form
                on:submit=on_submit
                class="w-96 h-fit border border-blue-100 rounded-lg px-6 py-6 flex flex-col gap-y-4">
                <div class="flex gap-1 items-center">
                    <img class="h-7 w-auto" src="/assets/logo.svg" alt="iDesoft Logo" />
                    <span class="font-bold text-blue-950 text-base">F5A Leptos</span>
                </div>

                <h1 class="text-2xl font-extrabold text-blue-950">Sign in</h1>

                <div class="flex flex-col gap-y-2">
                    <label class="text-blue-950" for="username">Username</label>
                    <input
                        id="username"
                        name="username"
                        autocomplete="username"
                        class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950"
                        type="text" />
                </div>

                <div class="flex flex-col gap-y-2">
                    <label class="text-blue-950" for="password">Password</label>
                    <input
                        id="password"
                        name="password"
                        autocomplete="current-password"
                        class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950"
                        type="password" />
                </div>

                <button
                    type="submit"
                    class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                    Login now
                </button>

            </form>
        </div>
    }
}
