use leptos::logging::log;
use leptos::prelude::*;

fn first_file_from(input: NodeRef<leptos::html::Input>) -> Option<web_sys::File> {
    input.get()?.files()?.get(0)
}

#[component]
pub fn UserCreatePage() -> impl IntoView {
    let file_ref = NodeRef::<leptos::html::Input>::new();

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        if let Some(file) = first_file_from(file_ref) {
            log!("file name: {}", file.name());
            log!("file size: {}", file.size());
            log!("file type: {}", file.type_());
        }

        // let file = file_ref
        //     .get()
        //     .and_then(|el| el.files())
        //     .and_then(|files| files.get(0));

        // if let Some(el_ref) = file_ref.get() {
        //     if let Some(files) = el_ref.files() {
        //         if let Some(file) = files.get(0) {
        //             log!("file name: {}", file.name());
        //             log!("file size: {}", file.size());
        //             log!("file type: {}", file.type_());
        //         }
        //     }
        // }
    };

    view! {

        <form on:submit=on_submit class="flex flex-col gap-y-4">
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">User Create</h1>

            <div class="flex flex-col gap-y-2">
                <input
                    node_ref=file_ref
                    id="avatar"
                    class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950"
                    type="file" />
            </div>

            <button
                type="submit"
                class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                Save
            </button>
        </form>
    }
}
