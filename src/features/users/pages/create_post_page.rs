use crate::features::users::components::FileListPreview;
use crate::features::users::models::FileItem;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use uuid::Uuid;

#[component]
pub fn CreatePostPage() -> impl IntoView {
    let (files, files_tx) = signal::<Vec<FileItem>>(Vec::new());

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();

        let form = ev
            .target()
            .unwrap()
            .unchecked_into::<web_sys::HtmlFormElement>();

        let data = web_sys::FormData::new_with_form(&form).unwrap();
        let message = data.get("message").as_string().unwrap_or_default();

        let file_entries = data.get_all("files");
        let files = file_entries
            .iter()
            .filter_map(|item| item.dyn_into::<web_sys::File>().ok())
            .collect::<Vec<web_sys::File>>();
        log!("post with message: {} and files: {:?}", message, files);
    };

    let on_file_change = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlInputElement>(&ev);

        if let Some(files) = target.files() {
            let file_items = (0..files.length())
                .filter_map(|it| files.get(it))
                .map(|file| FileItem {
                    id: Uuid::new_v4(),
                    name: file.name(),
                    size: file.size(),
                })
                .collect::<Vec<FileItem>>();

            files_tx.set(file_items);
        }
    };

    view! {
        <form
            on:submit=on_submit
            class="flex flex-col gap-y-4">
            <h1 class="text-2xl font-extrabold text-blue-950 dark:text-white">Create Post</h1>

            <div class="flex flex-col gap-y-2">
                <label class="text-blue-950 dark:text-white" for="message">"What's happening?"</label>
                <input
                    id="message"
                    name="message"
                    autocomplete="off"
                    class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950 dark:text-white dark:bg-slate-700 dark:border-slate-700"
                    type="text" />
            </div>

            <div class="flex flex-col gap-y-2">
                <input
                    on:change=on_file_change
                    multiple
                    name="files"
                    class="py-2 px-1.5 rounded-md border bg-white border-blue-100 text-blue-950"
                    type="file" />
            </div>

            <FileListPreview files=files />

            <button
                type="submit"
                class="bg-blue-950 text-white px-2 py-1.5 rounded-md hover:bg-blue-800">
                Publish
            </button>
        </form>
    }
}
