use crate::{core::forms::FileItem, features::users::components::FileItemPreview};
use leptos::prelude::*;

#[component]
pub fn FileListPreview(files: ReadSignal<Vec<FileItem>>) -> impl IntoView {
    view! {
        <ul class="flex flex-col gap-y-2">
            <For
                each=move || files.get()
                key=|item| item.id
                let(item)>
                <FileItemPreview file=item />
            </For>
        </ul>
    }
}
