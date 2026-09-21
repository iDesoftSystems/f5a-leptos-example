use leptos::prelude::event_target;
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub struct FileItem {
    pub id: Uuid,
    pub name: String,
    pub size: f64,
    pub type_: String,
}

impl From<web_sys::File> for FileItem {
    fn from(value: web_sys::File) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name(),
            size: value.size(),
            type_: value.type_(),
        }
    }
}

pub fn collect_file_items(ev: &web_sys::Event) -> Vec<FileItem> {
    let target = event_target::<web_sys::HtmlInputElement>(ev);
    target
        .files()
        .map(|files| {
            (0..files.length())
                .filter_map(|it| files.get(it))
                .map(FileItem::from)
                .collect()
        })
        .unwrap_or_default()
}
