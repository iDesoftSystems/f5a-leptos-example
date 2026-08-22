use crate::features::users::models::FileItem;
use leptos::prelude::{ReadSignal, Set, event_target, signal};
use uuid::Uuid;

pub fn use_file_change() -> (ReadSignal<Vec<FileItem>>, impl Fn(web_sys::Event)) {
    let (files, files_tx) = signal::<Vec<FileItem>>(Vec::new());

    let on_change = move |ev: web_sys::Event| {
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

    (files, on_change)
}
