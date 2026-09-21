use leptos::prelude::*;

use crate::core::forms::FileItem;

#[component]
pub fn FileItemPreview(file: FileItem) -> impl IntoView {
    view! {
        <li class="flex flex-col gap-y-1 px-2 py-2 border border-blue-100 rounded-md dark:border-slate-700 bg-white dark:bg-slate-700">
            <div class="text-base text-blue-950 dark:text-slate-300">{file.name}</div>
            <div class="text-sm text-blue-800 dark:text-slate-400">"Size: " {file.size} " Bytes"</div>
        </li>
    }
}
