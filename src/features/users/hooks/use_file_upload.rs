use leptos::prelude::{ReadSignal, Set, signal};

use crate::core::forms::{FileItem, collect_file_items};

pub fn use_file_change() -> (ReadSignal<Vec<FileItem>>, impl Fn(web_sys::Event)) {
    let (files, files_tx) = signal::<Vec<FileItem>>(Vec::new());

    let on_change = move |ev: web_sys::Event| {
        files_tx.set(collect_file_items(&ev));
    };

    (files, on_change)
}
